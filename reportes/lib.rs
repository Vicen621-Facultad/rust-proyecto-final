#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]

#[ink::contract]
mod reportes {
    use votacion::UserManager;
    use votacion::Usuario;
    use votacion::VotacionRef;
    use votacion::VotacionError;
    use votacion::ReportMessage;
    use ink::prelude::vec::Vec;
    type Result<T> = core::result::Result<T, VotacionError>;

    #[derive(Debug, Clone, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct DataRegistroVotantes {
        votantes: Vec<Usuario>
    }

    impl DataRegistroVotantes {
        fn new(votantes: Vec<Usuario>) -> DataRegistroVotantes {
            DataRegistroVotantes{
                votantes
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct DataParticipacion {
        votos: u32,
        participacion: u128
    }

    impl DataParticipacion {
        fn new(votos: u32, participacion: u128) -> DataParticipacion {
            DataParticipacion{
                votos,
                participacion
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct DataResultado {
        resultado: Vec<(AccountId, u32)>
    }

    impl DataResultado {
        fn new(resultado: Vec<(AccountId, u32)>) -> DataResultado {
            DataResultado {
                resultado
            }
        }
    }

    // ASK: Preguntar si se tienen que guardar los datos o con solo devolverlos ya estaria
    #[ink(storage)]
    pub struct Reportes {
        votacion: VotacionRef
    }

    impl Reportes {
        /// Crea un nuevo contrato de reportes
        #[ink(constructor)]
        pub fn new(votacion: VotacionRef) -> Self {
            Self { 
                votacion
            }
        }

        /// Crea un nuevo contrato de reportes a partir del hash de la votacion
        #[ink(constructor)]
        pub fn new_hash(votacion_hash: Hash) -> Self {
            let votacion = VotacionRef::new(Self::env().account_id())
                .endowment(0)
                .code_hash(votacion_hash)
                .salt_bytes(AccountId::from([0x63; 32]))
                .instantiate();

            Self { votacion }
        }

        /// Devuelve el reporte de registro de votantes de una eleccion
        #[ink(message)]
        pub fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<DataRegistroVotantes> {
            let id_votantes = self.votacion.reporte_registro_votantes(eleccion_id)?;
            let mut usuarios_votantes = Vec::new();

            // Itero sobre los id de los votantes para recuperar su usuario en el sistema y devolverlo en el reporte
            // Jamas deberia dar error el get_usuario(id) debido a que se verifica siempre que sean usuarios
            // aceptados aquellos que se los acepte como votantes y los candidadtos
            for id in id_votantes {
                usuarios_votantes.push(self.votacion.get_usuario(id)?);
            }

            Ok(DataRegistroVotantes::new(usuarios_votantes))
        }

        /// Devuelve el reporte de participacion de una eleccion
        #[ink(message)]
        pub fn reporte_participacion(&self, eleccion_id: u32) -> Result<DataParticipacion> {
            let data = self.votacion.reporte_participacion(eleccion_id)?;
            let num_votantes = data.0;
            let num_votantes_voto = data.1;

            if num_votantes == 0 {
                return Ok(DataParticipacion::new(0, 0));
            }

            let participacion = (num_votantes_voto * 100) / num_votantes;
            Ok(DataParticipacion::new(num_votantes_voto as u32, participacion))
        }

        /// Devuelve el reporte de resultado de una eleccion
        #[ink(message)]
        pub fn reporte_resultado(&self, eleccion_id: u32) -> Result<DataResultado> {
            let mut data = self.votacion.reporte_resultado(eleccion_id)?;
            data.sort_by_key(|(_, voto)| *voto);
            Ok(DataResultado::new(data))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_new_data_registro_votantes() {
            let data = DataRegistroVotantes::new(vec![Usuario::new(AccountId::from([0x1; 32]), "test".to_string(), "test".to_string(), "direccion".to_string(), "12345678".to_string(), 18)]);
            assert_eq!(data.votantes.len(), 1);
        }

        #[test]
        fn test_new_data_participacion() {
            let data = DataParticipacion::new(1, 1);
            assert_eq!(data.votos, 1);
            assert_eq!(data.participacion, 1);
        }

        #[test]
        fn test_new_data_resultado() {
            let data = DataResultado::new(vec![(AccountId::from([0x1; 32]), 1)]);
            assert_eq!(data.resultado.len(), 1);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use std::fmt::Write;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn test_reporte_registro_votantes<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let votacion = client
                .upload("votacion", &ink_e2e::alice())
                .submit()
                .await
                .expect("votacion `accumulator` failed")
                .code_hash;
            let mut constructor = ReportesRef::new_hash(votacion);

            let reportes = client
                .instantiate("reportes", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder =
                reportes.call_builder::<Reportes>();

            let reporte_registro_votantes = call_builder.reporte_registro_votantes(0);

            Ok(())
        }
    }
}
