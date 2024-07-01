#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reportes {
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
        #[ink(constructor)]
        pub fn new(votacion: VotacionRef) -> Self {
            Self { 
                votacion
            }
        }

        #[ink(message)]
        pub fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<DataRegistroVotantes> {
            let data = self.votacion.reporte_registro_votantes(eleccion_id)?;
            Ok(DataRegistroVotantes::new(data))
        }

        #[ink(message)]
        pub fn reporte_participacion(&self, eleccion_id: u32) -> Result<DataParticipacion> {
            let data = self.votacion.reporte_participacion(eleccion_id)?;
            Ok(DataParticipacion::new(data.0, data.1))
        }

        #[ink(message)]
        pub fn reporte_resultado(&self, eleccion_id: u32) -> Result<DataResultado> {
            let data = self.votacion.reporte_resultado(eleccion_id)?;
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
}
