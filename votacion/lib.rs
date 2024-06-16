#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]
pub use self::votacion::VotacionRef;

#[ink::contract]
mod votacion {
    use crate::errors::VotacionError;
    use crate::fecha::Fecha;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    type Result<T> = core::result::Result<T, VotacionError>;

    #[derive(Debug, Clone, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Eleccion {
        id: u32,
        votantes: Vec<AccountId>,
        candidatos: Vec<AccountId>,
        fecha_inicio: Fecha,
        fecha_fin: Fecha,
    }

    #[derive(Debug, Clone, PartialEq)]
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

    pub trait GettersEleccion {
        /// Devuelve el id de la elección
        fn get_id(&self) -> u32;
        /// Dado un id , devuelve true si esta registrado como votante en la eleccion
        fn is_votante(&self, id: &AccountId) -> bool;
        /// Dado un id , devuelve true si esta registrado como candidato en la eleccion
        fn is_candidato(&self, id: &AccountId) -> bool;
        /// Devuelve la fecha de inicio de la elección
        fn get_fecha_inicio(&self) -> Fecha;
        /// Devuelve la fecha de fin de la elección
        fn get_fecha_fin(&self) -> Fecha;
    }

    pub trait GettersUsuario {
        /// Devuelve el id del usuario
        fn get_addres(&self) -> AccountId;
        /// Devuelve el nombre del usuario
        fn get_nombre(&self) -> String;
        /// Devuelve el apellido del usuario
        fn get_apellido(&self) -> String;
        /// Devuelve la direccion del usuario
        fn get_direccion(&self) -> String;
        /// Devuelve el dni del usuario
        fn get_dni(&self) -> String;
        /// Devuelve la edad del usuario
        fn get_edad(&self) -> u8;
    }

    #[ink::trait_definition]
    pub trait UserManager {
        /// Devuelve el admin del contrato
        #[ink(message)]
        fn get_admin(&self) -> AccountId;
        /// Devuelve true si el id pasado es el admin del contrato, false en cualquier otro caso
        #[ink(message)]
        fn is_admin(&self, other: AccountId) -> bool;
        /// Crea un usuario y lo agrega a la lista de usuarios_por_aceptar
        #[ink(message)]
        fn crear_usuario(&mut self, id: AccountId, nombre: String, apellido: String, direccion: String, dni: String, edad: u8) -> Result<Usuario>;
        /// Acepta un usuario de la lista usuarios_por_aceptar y lo agrega a la lista de usuarios
        #[ink(message)]
        fn aceptar_usuario(&mut self, id: AccountId) -> Result<()>;
        /// Obtiene un usuario sin aceptar por su id
        #[ink(message)]
        fn get_usuario_sin_aceptar(&self, id: AccountId) -> Result<Usuario>;
        /// Obtiene un usuario por su id
        #[ink(message)]
        fn get_usuario(&self, id: AccountId) -> Result<Usuario>;
    }

    #[ink::trait_definition]
    pub trait EleccionImpl {
        /// Agrega un candidato a la elección
        #[ink(message)]
        fn agregar_candidato(&mut self, id_candidato: AccountId) -> Result<()>;
        /// Agrega un votante a la eleccion
        #[ink(message)]
        fn agregar_votante(&mut self, id_votante: AccountId) -> Result<()>;
        /// Devuelve true si el id pasado esta registrado como votante, false en cualquier otro caso
        #[ink(message)]
        fn es_votante(&self, id_votante: AccountId) -> bool;
        /// Devuelve true si el id pasado esta registrado como candidato, false en cualquier otro caso
        #[ink(message)]
        fn es_candidato(&self, id_candidato: AccountId) -> bool;
        /// Vota un usuario por un candidato
        #[ink(message)]
        fn votar(&mut self, id_votante: AccountId, id_candidato: AccountId) -> Result<()>;
        /// Inicia la elección
        #[ink(message)]
        fn iniciar(&mut self) -> Result<()>;
        /// Finaliza la elección
        #[ink(message)]
        fn finalizar(&mut self) -> Result<()>;
        /// Devuelve si la elección ya inició
        #[ink(message)]
        fn get_inicio(&self) -> bool;
        /// Devuelve si la elección ya finalizó
        #[ink(message)]
        fn get_finalizada(&self) -> bool;
        /// Devuelve los votos de un candidato
        #[ink(message)]
        fn get_votos_candidato(&self, id_candidato: AccountId) -> Result<u32>;
        // Devuelve los votos de todos los candidatos, almacenados por id
        // #[ink(message)]
        // fn get_votos(&self) -> Mapping<String, u32>;
    }

    #[ink::trait_definition]
    pub trait VotacionImpl {
        /// Crea una elección y la agrega a la lista de elecciones
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Fecha, fecha_fin: Fecha) -> Result<Eleccion>;
        /// Devuelve una elección por su ID
        #[ink(message)]
        fn get_eleccion(&self, id: u32) -> Option<Eleccion>;
    }

    #[ink(storage)]
    pub struct Votacion {
        admin: AccountId,
        elecciones: Vec<Eleccion>,
        usuarios: Vec<Usuario>,
        usuarios_sin_aceptar: Vec<Usuario>,
    }

    impl Eleccion {
        pub fn new(id: u32, fecha_inicio: Fecha, fecha_fin: Fecha) -> Self {
            Eleccion {
                id,
                votantes: Vec::new(),
                candidatos: Vec::new(),
                fecha_inicio,
                fecha_fin,
            }
        }
    }

    impl GettersEleccion for Eleccion {
        fn get_id(&self) -> u32 {
            self.id
        }

        fn is_votante(&self, id: &AccountId) -> bool {
            self.votantes.iter().any(|votante| votante == id)
        }

        fn is_candidato(&self, id: &AccountId) -> bool {
            self.candidatos.iter().any(|candidato| candidato == id)
        }

        fn get_fecha_inicio(&self) -> Fecha {
            self.fecha_inicio.clone()
        }

        fn get_fecha_fin(&self) -> Fecha {
            self.fecha_fin.clone()
        }
    }

    impl Usuario {
        pub fn new(addres: AccountId, nombre: String, apellido: String, direccion: String, dni: String, edad: u8) -> Self {
            Usuario {
                addres,
                nombre,
                apellido,
                direccion,
                dni,
                edad
            }
        }
    }

    impl GettersUsuario for Usuario {
        fn get_addres(&self) -> AccountId {
            self.addres
        }

        fn get_nombre(&self) -> String {
            self.nombre.clone()
        }

        fn get_apellido(&self) -> String {
            self.apellido.clone()
        }

        fn get_direccion(&self) -> String {
            self.direccion.clone()
        }

        fn get_dni(&self) -> String {
            self.dni.clone()
        }

        fn get_edad(&self) -> u8 {
            self.edad
        }
    }

    impl Default for Votacion {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Votacion {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
                elecciones: Vec::new(),
                usuarios: Vec::new(),
                usuarios_sin_aceptar: Vec::new(),
            }
        }
    }

    impl VotacionImpl for Votacion {
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Fecha, fecha_fin: Fecha) -> Result<Eleccion> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if !fecha_inicio.es_fecha_valida() || !fecha_fin.es_fecha_valida() {
                return Err(VotacionError::FechaInvalida);
            }

            if fecha_inicio.es_mayor(&fecha_fin) {
                return Err(VotacionError::FechaInicioMayorQueFin);
            }

            let id = self.elecciones.len() as u32;
            let eleccion = Eleccion::new(
                id,
                fecha_inicio,
                fecha_fin,
            );
            self.elecciones.push(eleccion.clone());
            Ok(eleccion)
        }

        #[ink(message)]
        fn get_eleccion(&self, id: u32) -> Option<Eleccion> {
            self.elecciones.get(id as usize).cloned()
        }
    }

    impl UserManager for Votacion {
        #[ink(message)]
        fn get_admin(&self) -> AccountId {
            self.admin
        }

        #[ink(message)]
        fn is_admin(&self, other: AccountId) -> bool {
            self.get_admin() == other
        }

        #[ink(message)]
        fn crear_usuario(&mut self, id: AccountId, nombre: String, apellido: String, direccion: String, dni: String, edad: u8) -> Result<Usuario> {
            if self.get_usuario_sin_aceptar(id).is_ok() {
                return Err(VotacionError::UsuarioNoAceptado);
            }

            if self.get_usuario(id).is_ok() {
                return Err(VotacionError::UsuarioYaRegistrado);
            }

            let usuario = Usuario::new(
                id,
                nombre,
                apellido,
                direccion,
                dni,
                edad
            );
            self.usuarios_sin_aceptar.push(usuario.clone());
            Ok(usuario)
        }

        #[ink(message)]
        fn aceptar_usuario(&mut self, id: AccountId) -> Result<()>{
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if let Some(pos) = self.usuarios_sin_aceptar.iter().position(|usuario| usuario.addres == id) {
                self.usuarios.push(self.usuarios_sin_aceptar.remove(pos));
                Ok(())
            } else {
                Err(VotacionError::UsuarioSinAceptarNoEncontrado)
            }
        }

        #[ink(message)]
        fn get_usuario_sin_aceptar(&self, id: AccountId) -> Result<Usuario> {
            self.usuarios_sin_aceptar.iter().find(|usuario| usuario.addres == id).cloned().ok_or(VotacionError::UsuarioSinAceptarNoEncontrado)
        }

        #[ink(message)]
        fn get_usuario(&self, id: AccountId) -> Result<Usuario> {
            self.usuarios.iter().find(|usuario| usuario.addres == id).cloned().ok_or(VotacionError::UsuarioNoEncontrado)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::{set_caller, default_accounts};

        // Tests de GettersUsuario
        #[test]
        fn test_getters_usuario() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let usuario = Usuario {
                addres: accounts.bob,
                nombre: "Juan".to_string(),
                apellido: "Perez".to_string(),
                direccion: "Calle Falsa 123".to_string(),
                dni: "12345678".to_string(),
                edad: 30
            };

            assert_eq!(usuario.get_addres(), accounts.bob);
            assert_eq!(usuario.get_nombre(), "Juan".to_string());
            assert_eq!(usuario.get_apellido(), "Perez".to_string());
            assert_eq!(usuario.get_direccion(), "Calle Falsa 123".to_string());
            assert_eq!(usuario.get_dni(), "12345678".to_string());
            assert_eq!(usuario.get_edad(), 30);
        }

        // Tests de GettersEleccion
        #[test]
        fn test_getters_eleccion() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let votantes = vec![accounts.bob, accounts.alice];
            let candidatos = vec![accounts.charlie, accounts.django];
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2024);
            let eleccion = Eleccion {
                id: 0,
                votantes: votantes.clone(),
                candidatos: candidatos.clone(),
                fecha_inicio: fecha_inicio.clone(),
                fecha_fin: fecha_fin.clone(),
            };

            assert_eq!(eleccion.get_id(), 0);
            assert!(eleccion.is_votante(&accounts.bob));
            assert!(!eleccion.is_votante(&accounts.charlie));
            assert!(eleccion.is_candidato(&accounts.django));
            assert!(!eleccion.is_candidato(&accounts.alice));
            assert_eq!(eleccion.get_fecha_inicio(), fecha_inicio);
            assert_eq!(eleccion.get_fecha_fin(), fecha_fin);
        }

        // Tests de VotacionImpl
        #[ink::test]
        fn test_crear_eleccion() {
            let mut votacion = Votacion::new();
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2024);
            let eleccion = votacion.crear_eleccion(fecha_inicio.clone(), fecha_fin.clone()).unwrap();
            assert_eq!(eleccion.get_id(), 0);
            assert_eq!(eleccion.get_fecha_inicio(), fecha_inicio);
            assert_eq!(eleccion.get_fecha_fin(), fecha_fin);
        }

        #[ink::test]
        fn test_crear_eleccion_error_no_admin() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            
            set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::new();
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2024);
            set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_crear_eleccion_error_fecha_invalida() {
            let mut votacion = Votacion::new();
            let fecha_inicio = Fecha::new(32, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2024);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::FechaInvalida));
        }

        #[ink::test]
        fn test_crear_eleccion_error_fecha_inicio_mayor_que_fin() {
            let mut votacion = Votacion::new();
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2023);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::FechaInicioMayorQueFin));
        }

        #[ink::test]
        fn test_get_eleccion() {
            let mut votacion = Votacion::new();
            let eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            assert_eq!(votacion.get_eleccion(0).unwrap(), eleccion);
        }

        // Tests de UserManager
        #[ink::test]
        fn test_crear_usuario() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            assert_eq!(usuario.get_addres(), accounts.bob);
            assert_eq!(usuario.get_nombre(), "Juan".to_string());
            assert_eq!(usuario.get_apellido(), "Perez".to_string());
            assert_eq!(usuario.get_direccion(), "Calle Falsa 123".to_string());
            assert_eq!(usuario.get_dni(), "12345678".to_string());
            assert_eq!(usuario.get_edad(), 30);
        }

        #[ink::test]
        fn test_crear_usuario_error_usuario_no_aceptado() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_crear_usuario_error_usuario_ya_registrado() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.usuarios.push(Usuario::new(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30));
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioYaRegistrado));
        }

        #[ink::test]
        fn test_aceptar_usuario() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            assert_eq!(votacion.usuarios_sin_aceptar.len(), 0);
            assert_eq!(votacion.usuarios.len(), 1);
        }

        #[ink::test]
        fn test_aceptar_usuario_error_no_admin() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let usuario = votacion.aceptar_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_aceptar_usuario_error_usuario_sin_aceptar_no_encontrado() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let usuario = votacion.aceptar_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioSinAceptarNoEncontrado));
        }

        #[ink::test]
        fn test_get_usuario_sin_aceptar() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            assert_eq!(votacion.get_usuario_sin_aceptar(accounts.bob).unwrap().get_addres(), accounts.bob);
        }

        #[ink::test]
        fn test_get_usuario_sin_aceptar_error_usuario_sin_aceptar_no_encontrado() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let votacion = Votacion::new();
            let usuario = votacion.get_usuario_sin_aceptar(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioSinAceptarNoEncontrado));
        }

        #[ink::test]
        fn test_get_usuario() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            assert_eq!(votacion.get_usuario(accounts.bob).unwrap().get_addres(), accounts.bob);
        }

        #[ink::test]
        fn test_get_usuario_error_usuario_no_encontrado() {
            let accounts =
                default_accounts::<ink::env::DefaultEnvironment>();
            let votacion = Votacion::new();
            let usuario = votacion.get_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoEncontrado));
        }

        // Tests de EleccionImpl
    }
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

    #[derive(Debug, Clone)]
    pub struct FechaError;

    impl core::fmt::Display for FechaError {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "Fecha inválida")
        }
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

mod errors {
    #[derive(Debug, Clone, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub enum VotacionError {
        FechaInvalida,
        FechaInicioMayorQueFin,
        NoEsAdmin,
        UsuarioYaRegistrado,
        UsuarioNoRegistrado,
        UsuarioSinAceptarNoEncontrado,
        UsuarioNoAceptado,
        UsuarioNoEncontrado,
        EleccionNoEncontrada,
        EleccionYaIniciada,
        EleccionYaFinalizada,
        UsuarioNoEsVotante,
        UsuarioNoEsCandidato,
    }

    impl core::fmt::Display for VotacionError {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                VotacionError::FechaInvalida => write!(f, "Fecha inválida"),
                VotacionError::FechaInicioMayorQueFin => write!(f, "La fecha de inicio es mayor que la fecha de fin"),
                VotacionError::NoEsAdmin => write!(f, "No es admin"),
                VotacionError::UsuarioYaRegistrado => write!(f, "Usuario ya registrado"),
                VotacionError::UsuarioNoRegistrado => write!(f, "Usuario no registrado"),
                VotacionError::UsuarioSinAceptarNoEncontrado => write!(f, "Usuario sin aceptar no encontrado"),
                VotacionError::UsuarioNoAceptado => write!(f, "Usuario no aceptado"),
                VotacionError::UsuarioNoEncontrado => write!(f, "Usuario no encontrado"),
                VotacionError::EleccionNoEncontrada => write!(f, "Elección no encontrada"),
                VotacionError::EleccionYaIniciada => write!(f, "Elección ya iniciada"),
                VotacionError::EleccionYaFinalizada => write!(f, "Elección ya finalizada"),
                VotacionError::UsuarioNoEsVotante => write!(f, "Usuario no es votante"),
                VotacionError::UsuarioNoEsCandidato => write!(f, "Usuario no es candidato"),
            }
        }
    }
}