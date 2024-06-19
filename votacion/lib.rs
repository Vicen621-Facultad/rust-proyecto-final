#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]
pub use self::votacion::VotacionRef;

#[ink::contract]
mod votacion {
    use crate::errors::VotacionError;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    type Result<T> = core::result::Result<T, VotacionError>;

    #[derive(Debug, Clone, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Eleccion {
        id: u32,
        votantes: Vec<AccountId>,
        candidatos: Vec<AccountId>,
        votantes_voto: Vec<AccountId>,
        votos: Vec<(AccountId, u32)>,
        fecha_inicio: Timestamp,
        fecha_fin: Timestamp,
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
        fn get_fecha_inicio(&self) -> Timestamp;
        /// Devuelve la fecha de fin de la elección
        fn get_fecha_fin(&self) -> Timestamp;
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
    pub trait EleccionSystemInk {
        /// Agrega un candidato a la elección
        #[ink(message)]
        fn agregar_candidato(&mut self, id_eleccion: u32, id_candidato: AccountId) -> Result<()>;
        /// Agrega un votante a la eleccion
        #[ink(message)]
        fn agregar_votante(&mut self, id_eleccion: u32, id_votante: AccountId) -> Result<()>;
        /// Vota un usuario por un candidato
        #[ink(message)]
        fn votar(&mut self, id_eleccion: u32, id_votante: AccountId, id_candidato: AccountId) -> Result<()>;
        /// Devuelve true si el usuario ya voto, false en cualquier otro caso
        #[ink(message)]
        fn ya_voto(&self, id_eleccion: u32, id_votante: AccountId) -> Result<bool>;
        /// Devuelve si la elección ya inició
        #[ink(message)]
        fn get_iniciada(&self, id_eleccion: u32) -> Result<bool>;
        /// Devuelve si la elección ya finalizó
        #[ink(message)]
        fn get_finalizada(&self, id_eleccion: u32) -> Result<bool>;
        /// Devuelve los votos de un candidato
        #[ink(message)]
        fn get_votos_candidato(&self, id_eleccion: u32, id_candidato: AccountId) -> Result<u32>;
    }

    pub trait EleccionSystem {
        /// Agrega un candidato a la elección
        fn agregar_candidato(&mut self, id_candidato: AccountId, current_time: Timestamp) -> Result<()>;
        /// Agrega un votante a la eleccion
        fn agregar_votante(&mut self, id_votante: AccountId, current_time: Timestamp) -> Result<()>;
        /// Vota un usuario por un candidato
        fn votar(&mut self, id_votante: &AccountId, id_candidato: &AccountId, current_time: Timestamp) -> Result<()>;
        /// Devuelve true si el usuario ya voto, false en cualquier otro caso
        fn ya_voto(&self, id_votante: &AccountId) -> bool;
        /// Devuelve si la elección ya inició
        fn get_inicio(&self, current_time: Timestamp) -> bool;
        /// Devuelve si la elección ya finalizó
        fn get_finalizada(&self, current_time: Timestamp) -> bool;
        /// Devuelve los votos de un candidato
        fn get_votos_candidato(&self, id_candidato: &AccountId, current_time: Timestamp) -> Result<u32>;
        // Devuelve los votos de todos los candidatos, almacenados por id
        // fn get_votos(&self) -> Vec<(AccountId, u32)>;
    }
    
    #[ink::trait_definition]
    pub trait EleccionManager {
        /// Crea una elección y la agrega a la lista de elecciones, devuelve su id
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Timestamp, fecha_fin: Timestamp) -> Result<u32>;
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
        pub fn new(id: u32, fecha_inicio: Timestamp, fecha_fin: Timestamp) -> Self {
            Eleccion {
                id,
                votantes: Vec::new(),
                candidatos: Vec::new(),
                votantes_voto: Vec::new(),
                votos: Vec::new(),
                fecha_inicio,
                fecha_fin,
            }
        }
    }
    
    impl EleccionSystem for Eleccion {
        fn agregar_candidato(&mut self, id_candidato: AccountId, current_time: Timestamp) -> Result<()> {    
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if self.get_inicio(current_time) {
                return Err(VotacionError::EleccionYaIniciada);
            }

            if self.is_candidato(&id_candidato) {
                return Err(VotacionError::UsuarioEsCandidato);
            }

            if self.is_votante(&id_candidato) {
                return Err(VotacionError::UsuarioEsVotante);
            }

            self.candidatos.push(id_candidato);
            self.votos.push((id_candidato, 0));
            Ok(())
        }

        fn agregar_votante(&mut self, id_votante: AccountId, current_time: Timestamp) -> Result<()> {
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if self.get_inicio(current_time) {
                return Err(VotacionError::EleccionYaIniciada);
            }

            if self.is_votante(&id_votante) {
                return Err(VotacionError::UsuarioEsVotante);
            }

            if self.is_candidato(&id_votante) {
                return Err(VotacionError::UsuarioEsCandidato);
            }

            self.votantes.push(id_votante);
            Ok(())
        }

        fn votar(&mut self, id_votante: &AccountId, id_candidato: &AccountId, current_time: Timestamp) -> Result<()> {
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if !self.get_inicio(current_time) {
                return Err(VotacionError::EleccionNoIniciada);
            }

            if !self.is_votante(id_votante) {
                return Err(VotacionError::UsuarioNoEsVotante);
            }
           
            if !self.is_candidato(id_candidato) {
                return Err(VotacionError::UsuarioNoEsCandidato);
            }

            if self.ya_voto(id_votante) {
                return Err(VotacionError::UsuarioYaVoto);
            }

            self.votantes_voto.push(*id_votante);
            self.votos.iter_mut()
                .find(|(candidato, _)| candidato == id_candidato)
                .map(|(_, votos)| *votos += 1);
            Ok(())
        }

        fn ya_voto(&self, id_votante: &AccountId) -> bool {
            self.votantes_voto.iter().any(|votante| votante == id_votante)
        }

        fn get_inicio(&self, current_time: Timestamp) -> bool {
            // Si fecha_inicio <= fecha_act  -> true
            current_time >= self.get_fecha_inicio()
        }

        fn get_finalizada(&self, current_time: Timestamp) -> bool {
            // Si fecha_act > fecha_fin -> true
            current_time > self.get_fecha_fin()
        }

        fn get_votos_candidato(&self, id_candidato: &AccountId, current_time: Timestamp) -> Result<u32> {
            if !self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionNoFinalizada);
            }

            if self.is_candidato(id_candidato) {
                let votos_candidato = self.votos.iter()
                    .find(|(candidato, _)| candidato == id_candidato)
                    .map(|(_, votos)| *votos)
                    .unwrap_or(0);
                Ok(votos_candidato)
            } else {
                Err(VotacionError::UsuarioNoEsCandidato)
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

        fn get_fecha_inicio(&self) -> Timestamp {
            self.fecha_inicio
        }

        fn get_fecha_fin(&self) -> Timestamp {
            self.fecha_fin
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

        /// Funcion debug para cambiar el admin del contrato
        #[ink(message)]
        pub fn set_admin(&mut self, new_admin: AccountId) -> Result<()> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            self.admin = new_admin;
            Ok(())
        }
    }

    impl EleccionManager for Votacion {
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Timestamp, fecha_fin: Timestamp) -> Result<u32> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if fecha_inicio > fecha_fin {
                return Err(VotacionError::FechaInicioMayorQueFin);
            }

            let id = self.elecciones.len() as u32;
            let eleccion = Eleccion::new(
                id,
                fecha_inicio,
                fecha_fin,
            );
            self.elecciones.push(eleccion.clone());
            Ok(id)
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

    impl EleccionSystemInk for Votacion {
        #[ink(message)]
        fn agregar_candidato(&mut self, id_eleccion: u32, id_candidato: AccountId) -> Result<()> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if self.get_usuario(id_candidato).is_err() {
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.agregar_candidato(id_candidato, timestamp)
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        #[ink(message)]
        fn agregar_votante(&mut self, id_eleccion: u32, id_votante: AccountId) -> Result<()> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if self.get_usuario(id_votante).is_err() {
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.agregar_votante(id_votante, timestamp)
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        #[ink(message)]
        fn votar(&mut self, id_eleccion: u32, id_votante: AccountId, id_candidato: AccountId) -> Result<()> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if self.get_usuario(id_candidato).is_err() || self.get_usuario(id_votante).is_err(){
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.votar(&id_votante, &id_candidato, timestamp)
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        #[ink(message)]
        fn ya_voto(&self, id_eleccion: u32, id_votante: AccountId) -> Result<bool> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if self.get_usuario(id_votante).is_err(){
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                Ok(eleccion.ya_voto(&id_votante))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        #[ink(message)]
        fn get_iniciada(&self, id_eleccion: u32) -> Result<bool> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                Ok(eleccion.get_inicio(self.env().block_timestamp()))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        #[ink(message)]
        fn get_finalizada(&self, id_eleccion: u32) -> Result<bool> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                Ok(eleccion.get_finalizada(self.env().block_timestamp()))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        #[ink(message)]
        fn get_votos_candidato(&self, id_eleccion: u32, id_candidato: AccountId) -> Result<u32> {
            if !self.is_admin(self.env().caller()) {
                return Err(VotacionError::NoEsAdmin);
            }

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                eleccion.get_votos_candidato(&id_candidato, self.env().block_timestamp())
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use chrono::{NaiveDate, NaiveDateTime};
        use ink::env::{test::{default_accounts, set_block_timestamp, set_caller}, DefaultEnvironment};

        /// Funcion de ayuda en los test le pasas un dia, mes y año
        /// te devuelve el timestamp de esa fecha
        fn create_date(day: u32, month: u32, year: i32) -> Timestamp {
            let date = NaiveDate::from_ymd_opt(year, month, day).expect("Fecha inválida");
            // Convertir la fecha a NaiveDateTime agregando tiempo (00:00:00)
            let datetime = NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            // Obtener el timestamp
            datetime.and_utc().timestamp() as u64
        }

        #[ink::test]
        fn test_set_admin() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.set_admin(accounts.bob).unwrap();
            assert_eq!(votacion.get_admin(), accounts.bob);
        }

        // Tests de GettersUsuario
        #[test]
        fn test_getters_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
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
                default_accounts::<DefaultEnvironment>();
            let votantes = vec![accounts.bob, accounts.alice];
            let candidatos = vec![accounts.charlie, accounts.django];

            let eleccion = Eleccion {
                id: 0,
                votantes: votantes.clone(),
                candidatos: candidatos.clone(),
                votos: vec![],
                votantes_voto: vec![],
                fecha_inicio: create_date(19, 6, 2024),
                fecha_fin: create_date(20, 6, 2024),
            };

            assert_eq!(eleccion.get_id(), 0);
            assert!(eleccion.is_votante(&accounts.bob));
            assert!(!eleccion.is_votante(&accounts.charlie));
            assert!(eleccion.is_candidato(&accounts.django));
            assert!(!eleccion.is_candidato(&accounts.alice));
            assert_eq!(eleccion.get_fecha_inicio(), create_date(19, 6, 2024));
            assert_eq!(eleccion.get_fecha_fin(), create_date(20, 6, 2024));
        }

        // Tests de VotacionImpl
        #[ink::test]
        fn test_crear_eleccion() {
            let mut votacion = Votacion::new();
            let fecha_inicio = create_date(1, 1, 2024);
            let fecha_fin = create_date(31, 12, 2024);
            let eleccion_id = votacion.crear_eleccion(fecha_inicio.clone(), fecha_fin.clone()).unwrap();
            let eleccion = votacion.get_eleccion(eleccion_id).unwrap();
            assert_eq!(eleccion.get_id(), 0);
            assert_eq!(eleccion.get_fecha_inicio(), fecha_inicio);
            assert_eq!(eleccion.get_fecha_fin(), fecha_fin);
        }

        #[ink::test]
        fn test_crear_eleccion_error_no_admin() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::new();
            let fecha_inicio = create_date(1, 1, 2024);
            let fecha_fin = create_date(31, 12, 2024);
            set_caller::<DefaultEnvironment>(accounts.alice);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_crear_eleccion_error_fecha_inicio_mayor_que_fin() {
            let mut votacion = Votacion::new();
            let fecha_inicio = create_date(1, 1, 2024);
            let fecha_fin = create_date(31, 12, 2023);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::FechaInicioMayorQueFin));
        }

        #[ink::test]
        fn test_get_eleccion() {
            let mut votacion = Votacion::new();
            let eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            assert_eq!(votacion.get_eleccion(0).unwrap().get_id(), eleccion);
        }

        // Tests de UserManager
        #[ink::test]
        fn test_crear_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
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
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_crear_usuario_error_usuario_ya_registrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.usuarios.push(Usuario::new(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30));
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioYaRegistrado));
        }

        #[ink::test]
        fn test_aceptar_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            assert_eq!(votacion.usuarios_sin_aceptar.len(), 0);
            assert_eq!(votacion.usuarios.len(), 1);
        }

        #[ink::test]
        fn test_aceptar_usuario_error_no_admin() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.charlie);
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let usuario = votacion.aceptar_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_aceptar_usuario_error_usuario_sin_aceptar_no_encontrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let usuario = votacion.aceptar_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioSinAceptarNoEncontrado));
        }

        #[ink::test]
        fn test_get_usuario_sin_aceptar() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            assert_eq!(votacion.get_usuario_sin_aceptar(accounts.bob).unwrap().get_addres(), accounts.bob);
        }

        #[ink::test]
        fn test_get_usuario_sin_aceptar_error_usuario_sin_aceptar_no_encontrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let votacion = Votacion::new();
            let usuario = votacion.get_usuario_sin_aceptar(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioSinAceptarNoEncontrado));
        }

        #[ink::test]
        fn test_get_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            assert_eq!(votacion.get_usuario(accounts.bob).unwrap().get_addres(), accounts.bob);
        }

        #[ink::test]
        fn test_get_usuario_error_usuario_no_encontrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let votacion = Votacion::new();
            let usuario = votacion.get_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoEncontrado));
        }

        //impl test de EleccionImpl
        #[ink::test]
        fn test_get_inicio_eleccion() {
            // Ya terminó
            let eleccion = Eleccion::new(0, create_date(1, 1, 2023), create_date(31, 12, 2023));
            assert!(eleccion.get_inicio(create_date(1, 1, 2024)));

            // Ya empezó y no terminó
            let eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            assert!(eleccion.get_inicio(create_date(15, 6, 2024)));

            // No empezó
            let eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(31, 12, 2025));
            assert!(!eleccion.get_inicio(create_date(1, 1, 2024)));
        }

        #[test]
        fn test_get_finalizada_eleccion() {
            // Ya terminó
            let eleccion = Eleccion::new(0, create_date(1, 1, 2023), create_date(31, 12, 2023));
            assert!(eleccion.get_finalizada(create_date(1, 1, 2024)));

            // Ya empezó y no terminó
            let eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            assert!(!eleccion.get_finalizada(create_date(15, 6, 2024)));

            // No empezó
            let eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(31, 12, 2025));
            assert!(!eleccion.get_inicio(create_date(1, 1, 2024)));
        }

        #[test]
        fn test_agregar_candidato_eleccion() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let candidato_id = AccountId::from([0x1; 32]);

            assert!(eleccion.agregar_candidato(candidato_id, create_date(1, 1, 2023)).is_ok());
            assert!(eleccion.is_candidato(&candidato_id));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let candidato_id = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_candidato(candidato_id, create_date(15, 6, 2024)), Err(VotacionError::EleccionYaIniciada));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(1, 1, 2024));
            let candidato_id = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_candidato(candidato_id, create_date(2, 1, 2024)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_candidato() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let candidato_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2024)).is_ok());
            assert_eq!(eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2024)), Err(VotacionError::UsuarioEsCandidato));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let candidato_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.agregar_votante(candidato_id, create_date(31, 12, 2024)).is_ok());
            assert_eq!(eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2024)), Err(VotacionError::UsuarioEsVotante));
        }
    
        #[test]
        fn test_agregar_votante_eleccion() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let votante_id = AccountId::from([0x2; 32]);
    
            assert!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2024)).is_ok());
            assert!(eleccion.is_votante(&votante_id));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
    
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(15, 6, 2024)), Err(VotacionError::EleccionYaIniciada));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_votante(votante_id, create_date(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_usuario_es_candidato() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.agregar_candidato(votante_id, create_date(31, 12, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)), Err(VotacionError::UsuarioEsCandidato));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_usuario_es_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)), Err(VotacionError::UsuarioEsVotante));
        }
    
        #[test]
        fn test_votar_eleccion() {
            let votante_id = AccountId::from([0x2; 32]);
            let candidato_id = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));

            eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();

            assert!(eleccion.votar(&votante_id, &candidato_id, create_date(15, 6, 2024)).is_ok());
            assert_eq!(eleccion.get_votos_candidato(&candidato_id, create_date(1, 1, 2025)), Ok(1));
        }
    
        #[test]
        fn test_votar_eleccion_error_eleccion_no_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
            let candidato_id = AccountId::from([0x1; 32]);
    
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &candidato_id, create_date(31, 12, 2023)), Err(VotacionError::EleccionNoIniciada));
        }
    
        #[test]
        fn test_votar_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
            let candidato_id = AccountId::from([0x1; 32]);
    
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &candidato_id, create_date(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_votar_eleccion_error_usuario_no_es_votante() {
            let candidato_id = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);

            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &candidato_id, create_date(15, 6, 2024)), Err(VotacionError::UsuarioNoEsVotante));
        }
    
        #[test]
        fn test_votar_eleccion_error_usuario_no_es_candidato() {
            let candidato_id = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);

            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &candidato_id, create_date(15, 6, 2024)), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[test]
        fn test_ya_voto_eleccion() {
            let votante_id = AccountId::from([0x2; 32]);
            let candidato_id = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));

            eleccion.agregar_candidato(candidato_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();

            assert!(!eleccion.ya_voto(&votante_id));
            eleccion.votar(&votante_id, &candidato_id, create_date(15, 6, 2024)).unwrap();
            assert!(eleccion.ya_voto(&votante_id));
        }

        // tests de EleccionSystemInk
        #[ink::test]
        fn test_agregar_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            assert!(votacion.agregar_candidato(eleccion_id, accounts.bob).is_ok());
            let eleccion = votacion.get_eleccion(eleccion_id).unwrap();
            assert!(eleccion.is_candidato(&accounts.bob));
        }

        #[ink::test]
        fn test_agregar_candidato_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            let resultado = votacion.agregar_candidato(eleccion_id, accounts.bob);
            assert_eq!(resultado, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_agregar_candidato_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            let resultado = votacion.agregar_candidato(eleccion_id, accounts.bob);
            assert_eq!(resultado, Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_agregar_votante_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            assert!(votacion.agregar_votante(eleccion_id, accounts.bob).is_ok());
            let eleccion = votacion.get_eleccion(eleccion_id).unwrap();
            assert!(eleccion.is_votante(&accounts.bob));
        }

        #[ink::test]
        fn test_agregar_votante_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            let resultado = votacion.agregar_votante(eleccion_id, accounts.bob);
            assert_eq!(resultado, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_agregar_votante_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            let resultado = votacion.agregar_votante(eleccion_id, accounts.bob);
            assert_eq!(resultado, Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_votar_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();
            votacion.agregar_candidato(eleccion_id, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));

            assert!(votacion.votar(eleccion_id, accounts.bob, accounts.alice).is_ok());
            let eleccion = votacion.get_eleccion(eleccion_id).unwrap();
            assert!(eleccion.ya_voto(&accounts.bob));
        }

        #[ink::test]
        fn test_votar_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();
            votacion.agregar_candidato(eleccion_id, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.votar(eleccion_id, accounts.bob, accounts.alice), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_votar_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            assert_eq!(votacion.votar(eleccion_id, accounts.bob, accounts.alice), Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_ya_voto_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();
            votacion.agregar_candidato(eleccion_id, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            votacion.votar(eleccion_id, accounts.bob, accounts.alice).unwrap();
            assert!(votacion.ya_voto(eleccion_id, accounts.bob).unwrap());
        }

        #[ink::test]
        fn test_ya_voto_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            assert_eq!(votacion.ya_voto(eleccion_id, accounts.alice), Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_ya_voto_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.ya_voto(eleccion_id, accounts.bob), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();
            votacion.agregar_candidato(eleccion_id, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            votacion.votar(eleccion_id, accounts.bob, accounts.alice).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(1, 1, 2025));
            assert_eq!(votacion.get_votos_candidato(eleccion_id, accounts.alice).unwrap(), 1);
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            votacion.agregar_votante(eleccion_id, accounts.bob).unwrap();
            votacion.agregar_candidato(eleccion_id, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            votacion.votar(eleccion_id, accounts.bob, accounts.alice).unwrap();


            set_block_timestamp::<DefaultEnvironment>(create_date(1, 1, 2025));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.get_votos_candidato(eleccion_id, accounts.alice), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_eleccion_no_encontrada() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let votacion = Votacion::new();
            assert_eq!(votacion.get_votos_candidato(0, accounts.bob), Err(VotacionError::EleccionNoEncontrada));
        }

        #[ink::test]
        fn test_get_iniciada_votacion() {
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            assert!(votacion.get_iniciada(eleccion_id).unwrap());
        }

        #[ink::test]
        fn test_get_iniciada_votacion_error_eleccion_no_encontrada() {
            let votacion = Votacion::new();
            assert_eq!(votacion.get_iniciada(0), Err(VotacionError::EleccionNoEncontrada));
        }

        #[ink::test]
        fn test_get_finalizada_votacion() {
            let mut votacion = Votacion::new();
            let eleccion_id = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(1, 1, 2025));
            assert!(votacion.get_finalizada(eleccion_id).unwrap());
        }

        #[ink::test]
        fn test_get_finalizada_votacion_error_eleccion_no_encontrada() {
            let votacion = Votacion::new();
            assert_eq!(votacion.get_finalizada(0), Err(VotacionError::EleccionNoEncontrada));
        }
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
        UsuarioEsVotante,
        UsuarioEsCandidato,
        EleccionNoIniciada,
        EleccionNoFinalizada,
        UsuarioYaVoto,
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
                VotacionError::UsuarioEsVotante => write!(f, "Usuario es votante"),
                VotacionError::UsuarioEsCandidato => write!(f, "Usuario es candidato"),
                VotacionError::EleccionNoIniciada => write!(f, "Elección no iniciada"),
                VotacionError::EleccionNoFinalizada => write!(f, "Elección no finalizada"),
                VotacionError::UsuarioYaVoto => write!(f, "Usuario ya voto"),
            }
        }
    }
}