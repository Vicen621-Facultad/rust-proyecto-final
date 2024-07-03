#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]

#[ink::contract]
mod reportes {
    #[cfg(not(test))]
    use votacion::{UserManager, ReportMessage};
    use votacion::Usuario;
    use votacion::VotacionRef;
    use votacion::VotacionError;
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
        porcentaje_participacion: u128
    }

    impl DataParticipacion {
        fn new(votos: u32, porcentaje_participacion: u128) -> DataParticipacion {
            DataParticipacion{
                votos,
                porcentaje_participacion
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

    #[ink(storage)]
    pub struct Reportes {
        #[cfg(not(test))]
        votacion: VotacionRef
    }

    impl Reportes {
        /// Crea un nuevo contrato de reportes
        #[ink(constructor)]
        #[cfg(not(test))]
        pub fn new(votacion: VotacionRef) -> Self {
            Self { 
                votacion
            }
        }

        #[cfg(test)]
        pub fn new() -> Self {
            Self {}
        }
        
        /// Devuelve la informacion necesaria para crear el reporte de los votantes registrados en una eleccion
        #[cfg(not(test))]
        fn data_reporte_registro_votantes(&self, eleccion_id: u32) -> Result<Vec<AccountId>> {
            self.votacion.reporte_registro_votantes(eleccion_id)
        }

        /// Devuelve la informacion necesaria para crear el reporte de la participacion en una eleccion
        #[cfg(not(test))]
        fn data_reporte_participacion(&self, eleccion_id: u32) -> Result<(u128, u128)> {
            self.votacion.reporte_participacion(eleccion_id)
        }

        /// Devuelve la informacion necesaria para crear el reporte del resultado de una eleccion
        #[cfg(not(test))]
        fn data_reporte_resultado(&self, eleccion_id: u32) -> Result<Vec<(AccountId, u32)>> {
            self.votacion.reporte_resultado(eleccion_id)
        }

        /// Devuelve un usuario a partir de su id
        #[cfg(not(test))]
        fn get_usuario(&self, id: AccountId) -> Result<Usuario> {
            self.votacion.get_usuario(id)
        }

        /// Funcion mockeada para devolver los votantes de una eleccion
        #[cfg(test)]
        fn data_reporte_registro_votantes(&self, _eleccion_id: u32) -> Result<Vec<AccountId>> {
            Ok(vec![AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), AccountId::from([0x3; 32])])
        }

        /// Funcion mockeada para devolver la informacion de la participacion de una eleccion
        #[cfg(test)]
        fn data_reporte_participacion(&self, _eleccion_id: u32) -> Result<(u128, u128)> {
            Ok((10, 4))
        }

        /// Funcion mockeada para devolver la informacion del resultado de una eleccion
        #[cfg(test)]
        fn data_reporte_resultado(&self, _eleccion_id: u32) -> Result<Vec<(AccountId, u32)>> {
            Ok(vec![(AccountId::from([0x1;32]), 2), (AccountId::from([0x2;32]), 0), (AccountId::from([0x3;32]), 1)])
        }

        /// Funcion mockeada para devolver un usuario
        #[cfg(test)]
        fn get_usuario(&self, id: AccountId) -> Result<Usuario> {
            Ok(Usuario::new(id, "test".to_string(), "test".to_string(), "direccion".to_string(), "12345678".to_string(), 18))
        }

        /// Crea y devuelve un nuevo reporte de los votantes registrados en una eleccion
        /// En caso de no haber votantes registrados, se devuelve un reporte con una lista vacia
        /// 
        /// # Errores
        /// Devuelve un error si la eleccion no es encontrada
        #[ink(message)]
        pub fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<DataRegistroVotantes> {
            let id_votantes = self.data_reporte_registro_votantes(eleccion_id)?;
            let mut usuarios_votantes = Vec::new();

            // Itero sobre los id de los votantes para recuperar su usuario en el sistema y devolverlo en el reporte
            // Jamas deberia dar error el get_usuario(id) debido a que se verifica siempre que sean usuarios
            // aceptados aquellos que se los acepte como votantes y los candidadtos
            for id in id_votantes {
                usuarios_votantes.push(self.get_usuario(id)?);
            }

            Ok(DataRegistroVotantes::new(usuarios_votantes))
        }

        /// Crea y devuelve un nuevo reporte de la participacion en una eleccion
        /// 
        /// # Errores
        /// - Devuelve un error si la eleccion no es encontrada
        /// - Devuelve un error si la eleccion no finalizó
        #[ink(message)]
        pub fn reporte_participacion(&self, eleccion_id: u32) -> Result<DataParticipacion> {
            let data = self.data_reporte_participacion(eleccion_id)?;
            let num_votantes = data.0;
            let num_votantes_voto = data.1;

            if num_votantes == 0 {
                return Ok(DataParticipacion::new(0, 0));
            }

            let participacion = (num_votantes_voto * 100) / num_votantes;
            Ok(DataParticipacion::new(num_votantes_voto as u32, participacion))
        }

        /// Crea y devuelve un nuevo reporte del resultado de una eleccion 
        /// ordenado por cantidad de votos de mayor a menor
        /// 
        /// # Errores
        /// - Devuelve un error si la eleccion no es encontrada
        /// - Devuelve un error si la eleccion no finalizó
        #[ink(message)]
        pub fn reporte_resultado(&self, eleccion_id: u32) -> Result<DataResultado> {
            let mut data = self.data_reporte_resultado(eleccion_id)?;
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
            assert_eq!(data.porcentaje_participacion, 1);
        }

        #[test]
        fn test_new_data_resultado() {
            let data = DataResultado::new(vec![(AccountId::from([0x1; 32]), 1)]);
            assert_eq!(data.resultado.len(), 1);
        }

        #[test]
        fn test_reporte_registro_votantes() {
            let reportes = Reportes::new();
            let data = reportes.reporte_registro_votantes(1).unwrap();
            assert_eq!(data.votantes.len(), 3);
        }

        #[test]
        fn test_reporte_participacion() {
            let reportes = Reportes::new();
            let data = reportes.reporte_participacion(1).unwrap();
            assert_eq!(data.votos, 4);
            assert_eq!(data.porcentaje_participacion, 40);
        }

        #[test]
        fn test_reporte_resultado() {
            let reportes = Reportes::new();
            let mut data = reportes.reporte_resultado(1).unwrap();
            assert_eq!(data.resultado.len(), 3);
            assert_eq!(data.resultado.pop(), Some((AccountId::from([0x1; 32]), 2)));
            assert_eq!(data.resultado.pop(), Some((AccountId::from([0x3; 32]), 1)));
            assert_eq!(data.resultado.pop(), Some((AccountId::from([0x2; 32]), 0)));
        }
    }
}
