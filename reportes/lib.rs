#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod reportes {
    use votacion::VotacionRef;
    use votacion::VotacionError;
    type Result<T> = core::result::Result<T, VotacionError>;

    #[ink(storage)]
    pub struct Reportes {
        votacion: VotacionRef,
    }

    #[ink::trait_definition]
    pub trait ReportMessage {
        /// Reporte de registro de votantes para una elección específica
        #[ink(message)]
        fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<u32>;
    
        /// Reporte de participación para una elección cerrada
        #[ink(message)]
        fn reporte_participacion(&self, eleccion_id: u32) -> Result<(u32, u128)>;
    
        /// Reporte de resultados finales de una elección cerrada
        #[ink(message)]
        fn reporte_resultado(&self, eleccion_id: u32) -> Result<Vec<(AccountId, u32)>>;
    }

    impl Reportes {
        #[ink(constructor)]
        pub fn new(votacion: VotacionRef) -> Self {
            Self { votacion }
        }
    }

    impl ReportMessage for Reportes {
        #[ink(message)]
        fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<u32> {
            self.votacion.reporte_registro_votantes(eleccion_id)
        }

        #[ink(message)]
        fn reporte_participacion(&self, eleccion_id: u32) -> Result<(u32, u128)> {
            self.votacion.reporte_participacion(eleccion_id)
        }

        #[ink(message)]
        fn reporte_resultado(&self, eleccion_id: u32) -> Result<Vec<(AccountId, u32)>> {
            self.votacion.reporte_resultado(eleccion_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        //TODO: Hacer tests
    }
}
