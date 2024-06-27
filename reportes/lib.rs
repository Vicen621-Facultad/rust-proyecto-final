#![cfg_attr(not(feature = "std"), no_std, no_main)]

//FIXME: Cuando intento instanciar este contrato me da error de code rejected
/*CodeRejected

The contract's code was found to be invalid during validation.The most likely cause of this is that an API was used which is not supported by thenode. This happens if an older node is used with a new version of ink!. Try updatingyour node to the newest available version.A more detailed error can be found on the node console if debug messages are enabledby supplying `-lruntime::contracts=debug`.

New code rejected on wasmi instantiation! */
//TODO: Hacer distintos structs para cada reporte y devolver ese struct
#[ink::contract]
mod reportes {
    use votacion::VotacionRef;
    use votacion::VotacionError;
    use votacion::ReportMessage;
    use ink::prelude::vec::Vec;
    type Result<T> = core::result::Result<T, VotacionError>;

    #[ink(storage)]
    pub struct Reportes {
        votacion: VotacionRef,
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
