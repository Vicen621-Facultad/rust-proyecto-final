#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]
pub use self::votacion::VotacionRef;

#[ink::contract]
mod votacion {
    use crate::fecha::Fecha;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Eleccion {
        id: u32,
        votantes: Vec<AccountId>,
        cadidatos: Vec<AccountId>,
        fecha_inicio: Fecha,
        fecha_fin: Fecha,
    }

    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Usuario {
        addres: AccountId,
        nombre: String,
        apellido: String,
        direccion: String,
        dni: String,
        edad: u8
    }

    #[ink::trait_definition]
    pub trait GettersEleccion {
        /// Dado un DNI de usuario, devuelve el usuario si es que existe
        #[ink(message)]
        fn get_usuario(&self, id: AccountId) -> Option<Usuario>;
        /// Devuelve la fecha de inicio de la elección
        #[ink(message)]
        fn get_fecha_inicio(&self) -> Fecha;
        /// Devuelve la fecha de fin de la elección
        #[ink(message)]
        fn get_fecha_fin(&self) -> Fecha;
    }

    #[ink::trait_definition]
    pub trait UserManager {
        /// Crea un usuario y lo agrega a la lista de usuarios_por_aceptar
        #[ink(message)]
        fn crear_usuario(&mut self, id: AccountId, nombre: String, apellido: String, direccion: String, dni: String, edad: u8) -> Option<Usuario>;
        /// Acepta un usuario de la lista usuarios_por_aceptar y lo agrega a la lista de usuarios
        #[ink(message)]
        fn aceptar_usuario(&mut self, id: AccountId);
    }

    #[ink::trait_definition]
    pub trait EleccionImpl {
        /// Agrega un candidato a la elección
        #[ink(message)]
        fn agregar_candidato(&mut self, id_candidato: AccountId);
        /// Agrega un votante a la eleccion
        #[ink(message)]
        fn agregar_votante(&mut self, id_votante: AccountId);
        /// Devuelve true si el id pasado esta registrado como votante, false en cualquier otro caso
        #[ink(message)]
        fn es_votante(&self, id_votante: AccountId) -> bool;
        /// Devuelve true si el id pasado esta registrado como candidato, false en cualquier otro caso
        #[ink(message)]
        fn es_candidato(&self, id_candidato: AccountId) -> bool;
        /// Vota un usuario por un candidato
        #[ink(message)]
        fn votar(&mut self, id_votante: AccountId, id_candidato: AccountId);
        /// Inicia la elección
        #[ink(message)]
        fn iniciar(&mut self);
        /// Finaliza la elección
        #[ink(message)]
        fn finalizar(&mut self);
        /// Devuelve si la elección ya inició
        #[ink(message)]
        fn get_inicio(&self) -> bool;
        /// Devuelve si la elección ya finalizó
        #[ink(message)]
        fn get_finalizada(&self) -> bool;
        /// Devuelve los votos de un candidato
        #[ink(message)]
        fn get_votos_candidato(&self, id_candidato: AccountId) -> u32;
        // Devuelve los votos de todos los candidatos, almacenados por DNI
        // #[ink(message)]
        // fn get_votos(&self) -> Mapping<String, u32>;
    }

    #[ink::trait_definition]
    pub trait VotacionImpl {
        /// Crea una elección y la agrega a la lista de elecciones
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Fecha, fecha_fin: Fecha) -> Option<Eleccion>;
        /// Devuelve una elección por su ID
        #[ink(message)]
        fn get_eleccion(&self, id: u32) -> Option<Eleccion>;
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Votacion {
        admin: AccountId,
        elecciones: Vec<Eleccion>,
        usuarios: Vec<Usuario>,
        usuarios_sin_aceptar: Vec<Usuario>,
    }

    impl Votacion {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
                elecciones: Vec::new(),
                usuarios: Vec::new(),
                usuarios_sin_aceptar: Vec::new(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        /*#[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }*/

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn get(&mut self) {
        }

        /*/// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }*/
    }

    /*/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let votacion = Votacion::default();
            assert_eq!(votacion.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut votacion = Votacion::new(false);
            assert_eq!(votacion.get(), false);
            votacion.flip();
            assert_eq!(votacion.get(), true);
        }
    }*/
}

mod fecha {

    #[derive(Debug, Clone, Eq, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Fecha {
        day: u32,
        month: u32,
        year: i32,
    }

    /*impl Default for Fecha {
        fn default() -> Self {
            Fecha::now()
        }
    }*/

    impl Fecha {
        /*pub fn now() -> Self {
            let now = Local::now();
            Fecha {
                day: now.day(),
                month: now.month(),
                year: now.year(),
            }
        }*/

        pub fn new(day: u32, month: u32, year: i32) -> Self {
            Fecha {
                day,
                month,
                year
            }
        }

        pub fn es_fecha_valida(&self) -> bool {
            self.day <= self.obtener_dias_para_mes() && self.day > 0
        }

        pub fn es_bisiesto(&self) -> bool {
            self.year % 4 == 0
        }

        /// Devuelve la cantidad de dias que tiene el mes actual
        fn obtener_dias_para_mes(&self) -> u32 {
            if self.month > 12 || self.month < 1 {
                return 0;
            }

            const DIAS_POR_MES: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let dias = DIAS_POR_MES[(self.month - 1) as usize];
            // bool as u32 = if true { 1 } else { 0 }
            dias + (self.month == 2 && self.es_bisiesto()) as u32
        }

        pub fn sumar_dias(&mut self, dias: u32) {
            let mut dias_restantes = dias;
            while dias_restantes > 0 {
                let dias_en_mes = self.obtener_dias_para_mes();
                // Se suma 1 ya que tengo que contar el dia actual
                let dias_hasta_fin_de_mes = dias_en_mes - self.day + 1;

                if dias_hasta_fin_de_mes > dias_restantes {
                    self.day += dias_restantes;
                    dias_restantes = 0;
                } else {
                    dias_restantes -= dias_hasta_fin_de_mes;
                    self.month += 1;
                    if self.month > 12 {
                        self.month = 1;
                        self.year += 1;
                    }
                    self.day = 1;
                }
            }
        }

        pub fn restar_dias(&mut self, dias: u32) {
            let mut dias_restantes = dias;
            while dias_restantes > 0 {
                if self.day > dias_restantes {
                    self.day -= dias_restantes;
                    dias_restantes = 0;
                } else {
                    dias_restantes -= self.day;
                    self.month -= 1;
                    if self.month == 0 {
                        self.month = 12;
                        self.year -= 1;
                    }
                    self.day = self.obtener_dias_para_mes();
                }
            }
        }

        pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
            (self.year > una_fecha.year) || 
                (self.year == una_fecha.year && self.month > una_fecha.month) || 
                (self.year == una_fecha.year && self.month == una_fecha.month && self.day > una_fecha.day)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_es_fecha_valida() {
            // Fecha válida
            let fecha_valida = Fecha::new(15, 6, 2024);
            assert!(fecha_valida.es_fecha_valida());

            // Fecha inválida (día fuera de rango)
            let fecha_invalida_dia = Fecha::new(32, 6, 2024);
            assert!(!fecha_invalida_dia.es_fecha_valida());

            // Fecha inválida (mes fuera de rango)
            let fecha_invalida_mes = Fecha::new(15, 13, 2024);
            assert!(!fecha_invalida_mes.es_fecha_valida());

            // Fecha inválida (febrero en anio no bisiesto)
            let fecha_invalida_febrero_no_bisiesto = Fecha::new(29, 2, 2023);
            assert!(!fecha_invalida_febrero_no_bisiesto.es_fecha_valida());

            // Fecha válida (febrero en anio bisiesto)
            let fecha_valida_febrero_bisiesto = Fecha::new(29, 2, 2024);
            assert!(fecha_valida_febrero_bisiesto.es_fecha_valida());
        }

        #[test]
        fn test_es_bisiesto() {
            // Anio bisiesto
            let fecha_bisiesto = Fecha::new(1, 1, 2024);
            assert!(fecha_bisiesto.es_bisiesto());

            // Anio no bisiesto
            let fecha_no_bisiesto = Fecha::new(1, 1, 2023);
            assert!(!fecha_no_bisiesto.es_bisiesto());
        }

        #[test]
        fn test_sumar_dias() {
            let mut fecha = Fecha::new(1, 1, 2024);
            fecha.sumar_dias(365);
            assert_eq!(fecha, Fecha::new(31, 12, 2024));
            fecha.sumar_dias(1);
            assert_eq!(fecha, Fecha::new(1, 1, 2025));
            fecha.sumar_dias(5);
            assert_eq!(fecha, Fecha::new(6, 1, 2025));
        }

        #[test]
        fn test_restar_dias() {
            let mut fecha = Fecha::new(31, 12, 2024);
            fecha.restar_dias(365);
            assert_eq!(fecha, Fecha::new(1, 1, 2024));
            fecha.restar_dias(1);
            assert_eq!(fecha, Fecha::new(31, 12, 2023));
            fecha.restar_dias(5);
            assert_eq!(fecha, Fecha::new(26, 12, 2023));
        }

        #[test]
        fn test_es_mayor() {
            let fecha1 = Fecha::new(5, 3, 2024);
            let fecha2 = Fecha::new(5, 3, 2023);
            assert!(fecha1.es_mayor(&fecha2));

            let fecha3 = Fecha::new(5, 3, 2023);
            let fecha4 = Fecha::new(5, 3, 2024);
            assert!(!fecha3.es_mayor(&fecha4));

            let fecha5 = Fecha::new(5, 4, 2024);
            let fecha6 = Fecha::new(5, 3, 2024);
            assert!(fecha5.es_mayor(&fecha6));

            let fecha7 = Fecha::new(5, 3, 2024);
            let fecha8 = Fecha::new(5, 4, 2024);
            assert!(!fecha7.es_mayor(&fecha8));

            let fecha9 = Fecha::new(6, 3, 2024);
            let fecha10 = Fecha::new(5, 3, 2024);
            assert!(fecha9.es_mayor(&fecha10));

            let fecha11 = Fecha::new(5, 3, 2024);
            let fecha12 = Fecha::new(6, 3, 2024);
            assert!(!fecha11.es_mayor(&fecha12));
        }

        /*#[test]
        fn test_now() {
            let fecha = Fecha::now();
            let now = Local::now();
            assert_eq!(fecha.day, now.day());
            assert_eq!(fecha.month, now.month());
            assert_eq!(fecha.year, now.year());
        }*/
    }
}
