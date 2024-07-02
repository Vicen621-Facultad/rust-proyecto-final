#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]
pub use self::votacion::{
    Votacion,
    VotacionRef,
    ReportMessage,
    UserManager,
    Usuario
};
pub use self::errors::VotacionError;

//TODO: Agregar mejores comentarios a los ink(message)
#[ink::contract]
mod votacion {
    use crate::errors::VotacionError;
    use crate::fecha::Fecha;
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
        votantes_sin_aceptar: Vec<AccountId>,
        candidatos_sin_aceptar:Vec<AccountId>,
        votantes: Vec<AccountId>,
        candidatos: Vec<AccountId>,
        votantes_voto: Vec<AccountId>,
        votos: Vec<(AccountId, u32)>,
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
        /// Devuelve true si el id esta postulado como candidato
        fn is_postulado_candidato(&self, id: &AccountId) -> bool;
        /// Devuelve true si el id esta postulado como votante
        fn is_postulado_votante(&self, id: &AccountId) -> bool;
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
        fn caller_is_admin(&self) -> bool;
        /// Crea un usuario y lo agrega a la lista de usuarios_por_aceptar
        #[ink(message)]
        fn crear_usuario(&mut self, nombre: String, apellido: String, direccion: String, dni: String, edad: u8) -> Result<Usuario>;
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
        /// Dado un id: postula un candidato, si este esta postulado como votante, devolvera error
        #[ink(message)]
        fn postular_candidato(&mut self, id_eleccion: u32) -> Result<()>;
        /// Dado un id: postula un votante, si este esta postulado como candidato, devolvera error
        #[ink(message)]
        fn postular_votante(&mut self, id_eleccion: u32) -> Result<()>;
        /// Dado un candidato postulado, es aceptado por el Admin
        #[ink(message)]
        fn agregar_candidato(&mut self, id_eleccion: u32, id_candidato: AccountId) -> Result<()>;
        /// Dado un votante postulado, es aceptado por el Admin
        #[ink(message)]
        fn agregar_votante(&mut self, id_eleccion: u32, id_votante: AccountId) -> Result<()>;
        /// Vota por un candidato en una eleccion
        #[ink(message)]
        fn votar(&mut self, id_eleccion: u32, id_candidato: AccountId) -> Result<()>;
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
        /// Postula un candidato, si este esta postulado como votante, devolvera error
        fn postular_candidato(&mut self, id_candidato: AccountId, current_time: &Fecha) -> Result<()>;
        /// Postula un votante, si este esta postulado como candidato, devolvera error
        fn postular_votante(&mut self, id_votante: AccountId, current_time: &Fecha) -> Result<()>;
        /// Dado un candidato postulado, es aceptado por el Admin
        fn agregar_candidato(&mut self, id_candidato: AccountId, current_time: &Fecha) -> Result<()>;
        /// Dado un votante postulado, es aceptado por el Admin
        fn agregar_votante(&mut self, id_votante: AccountId, current_time: &Fecha) -> Result<()>;
        /// Vota un usuario por un candidato
        fn votar(&mut self, id_votante: &AccountId, id_candidato: &AccountId, current_time: &Fecha) -> Result<()>;
        /// Devuelve true si el usuario ya voto, false en cualquier otro caso
        fn ya_voto(&self, id_votante: &AccountId) -> bool;
        /// Devuelve si la elección ya inició
        fn get_inicio(&self, current_time: &Fecha) -> bool;
        /// Devuelve si la elección ya finalizó
        fn get_finalizada(&self, current_time: &Fecha) -> bool;
        /// Devuelve la cantidad de votos para un candidato dado
        fn get_votos_candidato(&self, id_candidato: &AccountId, current_time: &Fecha) -> Result<u32>;
        /// Devuelve los votos de todos los candidatos, almacenados por id
        fn get_votos(&self, current_time: &Fecha) -> Result<Vec<(AccountId, u32)>>;
    }
    
    #[ink::trait_definition]
    pub trait EleccionManager {
        /// Crea una elección y la agrega a la lista de elecciones, devuelve su id
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Fecha, fecha_fin: Fecha) -> Result<u32>;
        /// Devuelve una elección por su ID
        #[ink(message)]
        fn get_eleccion(&self, id: u32) -> Option<Eleccion>;

        
    }

    #[ink(storage)]
    pub struct Votacion {
        admin: AccountId,
        reporte: AccountId,
        elecciones: Vec<Eleccion>,
        usuarios: Vec<Usuario>,
        usuarios_sin_aceptar: Vec<Usuario>,
    }

    impl Eleccion {
        pub fn new(id: u32, fecha_inicio: Fecha, fecha_fin: Fecha) -> Self {
            Eleccion {
                id,
                votantes_sin_aceptar: Vec::new(),
                candidatos_sin_aceptar: Vec::new(),
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
        fn postular_votante(&mut self, id_votante: AccountId, current_time: &Fecha) -> Result<()> {
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if self.get_inicio(current_time) {
                return Err(VotacionError::EleccionYaIniciada);
            }

            if self.is_postulado_candidato(&id_votante) {
                return Err(VotacionError::UsuarioEsPostuladoCandidato)
            }

            if self.is_postulado_votante(&id_votante) {
                return Err(VotacionError::UsuarioEsPostuladoVotante);
            }
            // Añado votante postulado
            self.votantes_sin_aceptar.push(id_votante);
            Ok(())
        }

        fn postular_candidato(&mut self, id_candidato: AccountId, current_time: &Fecha) -> Result<()> {
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if self.get_inicio(current_time) {
                return Err(VotacionError::EleccionYaIniciada);
            }

            if self.is_postulado_candidato(&id_candidato) {
                return Err(VotacionError::UsuarioEsPostuladoCandidato)
            }

            if self.is_postulado_votante(&id_candidato) {
                return Err(VotacionError::UsuarioEsPostuladoVotante);
            }

            // Añado candidato postulado
            self.candidatos_sin_aceptar.push(id_candidato);
            Ok(())
        }

        fn agregar_candidato(&mut self, id_candidato: AccountId, current_time: &Fecha) -> Result<()> {    
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

            //Transfiero el id desde 'candidatos sin aceptar de la eleccion', 
            //-> hacia 'candidatos de la eleccion' (candidatos aceptados por el admin)
            if let Some(pos) = self.candidatos_sin_aceptar.iter().position(|candidato| *candidato == id_candidato) {
                self.candidatos.push(self.candidatos_sin_aceptar.remove(pos));
                self.votos.push((id_candidato, 0));
                Ok(())
            } else {
                Err(VotacionError::UsuarioNoPostuladoCandidato)
            }
        }

        fn agregar_votante(&mut self, id_votante: AccountId, current_time: &Fecha) -> Result<()> {
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

            //Transfiero el id desde 'votantes sin aceptar de la eleccion', 
            //-> hacia 'votantes de la eleccion' (votantes aceptados por el admin)
            if let Some(pos) = self.votantes_sin_aceptar.iter().position(|votante| *votante == id_votante) {
                self.votantes.push(self.votantes_sin_aceptar.remove(pos));
                Ok(())
            } else {
                Err(VotacionError::UsuarioNoPostuladoVotante)
            }
        }

        fn votar(&mut self, id_votante: &AccountId, id_candidato: &AccountId, current_time: &Fecha) -> Result<()> {
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

        fn get_inicio(&self, current_time: &Fecha) -> bool {
            // Si current_time >= fecha_inicio   -> true
            // Le puedo hacer unwrap porque me fijo si es valida al momento de crear la eleccion
            current_time.timestamp().unwrap() >= self.get_fecha_inicio().timestamp().unwrap()
        }

        fn get_finalizada(&self, current_time: &Fecha) -> bool {
            // Si fecha_act > fecha_fin -> true
            // Le puedo hacer unwrap porque me fijo si es valida al momento de crear la eleccion
            current_time.timestamp().unwrap() > self.get_fecha_fin().timestamp().unwrap()
        }

        fn get_votos_candidato(&self, id_candidato: &AccountId, current_time: &Fecha) -> Result<u32> {
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

        fn get_votos(&self, current_time: &Fecha)  -> Result<Vec<(AccountId, u32)> > {
            if !self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionNoFinalizada);
            }
            Ok(self.votos.clone())
        }
    }

    impl GettersEleccion for Eleccion {
        /// Devuelve el id de la elección
        fn get_id(&self) -> u32 {
            self.id
        }

        /// Devuelve true si el id esta postulado como candidato
        fn is_postulado_candidato(&self, id: &AccountId) -> bool {
            self.candidatos_sin_aceptar.iter().any(|candidato| candidato == id)
        }

        /// Devuelve true si el id esta postulado como votante
        fn is_postulado_votante(&self, id: &AccountId) -> bool {
            self.votantes_sin_aceptar.iter().any(|votante| votante == id)
        }

        /// Dado un id , devuelve true si esta registrado como votante en la eleccion
        fn is_votante(&self, id: &AccountId) -> bool {
            self.votantes.iter().any(|votante| votante == id)
        }

        /// Dado un id , devuelve true si esta registrado como candidato en la eleccion
        fn is_candidato(&self, id: &AccountId) -> bool {
            self.candidatos.iter().any(|candidato| candidato == id)
        }

        /// Devuelve la fecha de inicio de la elección
        fn get_fecha_inicio(&self) -> Fecha {
            self.fecha_inicio.clone()
        }

        /// Devuelve la fecha de fin de la elección
        fn get_fecha_fin(&self) -> Fecha {
            self.fecha_fin.clone()
        }
    }

    impl ReportMessageEleccion for Eleccion {
        /// Devuelve un listado de los id de los votantes aprobados
        fn reporte_registro_votantes(&self) -> Vec<AccountId> {
            self.votantes.clone()
        }

        fn reporte_participacion(&self, current_time: &Fecha) -> Result<(u128, u128)> {
            if !self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionNoFinalizada);
            }

            let num_votantes = self.votantes.len() as u128;
            let num_votantes_voto = self.votantes_voto.len() as u128;

            Ok((num_votantes, num_votantes_voto))
        }

        fn reporte_resultado(&self, current_time: &Fecha) -> Result<Vec<(AccountId, u32)>> {
            let resultados = self.get_votos(current_time)?;
            Ok(resultados)
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
        /// Devuelve el id del usuario
        fn get_addres(&self) -> AccountId {
            self.addres
        }

        /// Devuelve el nombre del usuario
        fn get_nombre(&self) -> String {
            self.nombre.clone()
        }

        /// Devuelve el apellido del usuario
        fn get_apellido(&self) -> String {
            self.apellido.clone()
        }

        /// Devuelve la direccion del usuario
        fn get_direccion(&self) -> String {
            self.direccion.clone()
        }

        /// Devuelve el dni del usuario
        fn get_dni(&self) -> String {
            self.dni.clone()
        }

        /// Devuelve la edad del usuario
        fn get_edad(&self) -> u8 {
            self.edad
        }
    }

    impl Votacion {
        /// Constructor por defecto
        pub fn default() -> Self {
            Self {
                admin: Self::env().caller(),
                reporte: AccountId::from([0x10; 32]),
                elecciones: Vec::new(),
                usuarios: Vec::new(),
                usuarios_sin_aceptar: Vec::new()
            }
        }

        /// Constructor del contrato
        #[ink(constructor)]
        pub fn new(reporte: AccountId) -> Self {
            Self {
                admin: Self::env().caller(),
                reporte,
                elecciones: Vec::new(),
                usuarios: Vec::new(),
                usuarios_sin_aceptar: Vec::new()
            }
        }

        /// Cambia el admin del contrato
        #[ink(message)]
        pub fn set_admin(&mut self, new_admin: AccountId) -> Result<()> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            self.admin = new_admin;
            Ok(())
        }

        /// Cambia el reporte del contrato
        #[ink(message)]
        pub fn set_reporte(&mut self, new_reporte: AccountId) -> Result<()> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            self.reporte = new_reporte;
            Ok(())
        }

        /// Devuelve true si el caller es el reporte, false en cualquier otro caso
        fn caller_is_reporte(&self) -> bool {
            self.env().caller() == self.reporte
        }
    }

    impl EleccionManager for Votacion {
        /// Crea una elección y la agrega a la lista de elecciones, devuelve su id
        #[ink(message)]
        fn crear_eleccion(&mut self, fecha_inicio: Fecha, fecha_fin: Fecha) -> Result<u32> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            if !fecha_inicio.es_fecha_valida() || !fecha_fin.es_fecha_valida() {
                return Err(VotacionError::FechaInvalida);
            }

            if fecha_inicio.timestamp().unwrap() > fecha_fin.timestamp().unwrap() {
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

        /// Devuelve una elección por su ID
        #[ink(message)]
        fn get_eleccion(&self, id: u32) -> Option<Eleccion> {
            self.elecciones.get(id as usize).cloned()
        }
    }

    impl UserManager for Votacion {
        /// Devuelve el admin del contrato
        #[ink(message)]
        fn get_admin(&self) -> AccountId {
            self.admin
        }

        /// Devuelve true si el id pasado es el admin del contrato, false en cualquier otro caso
        #[ink(message)]
        fn caller_is_admin(&self) -> bool {
            self.get_admin() == self.env().caller()
        }

        /// Crea un usuario y lo agrega a la lista de usuarios_por_aceptar
        #[ink(message)]
        fn crear_usuario(&mut self, nombre: String, apellido: String, direccion: String, dni: String, edad: u8) -> Result<Usuario> {
            let id = self.env().caller();

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

        /// Acepta un usuario de la lista usuarios_por_aceptar y lo agrega a la lista de usuarios
        #[ink(message)]
        fn aceptar_usuario(&mut self, id: AccountId) -> Result<()>{
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            if let Some(pos) = self.usuarios_sin_aceptar.iter().position(|usuario| usuario.addres == id) {
                self.usuarios.push(self.usuarios_sin_aceptar.remove(pos));
                Ok(())
            } else {
                Err(VotacionError::UsuarioSinAceptarNoEncontrado)
            }
        }

        /// Obtiene un usuario sin aceptar por su id
        #[ink(message)]
        fn get_usuario_sin_aceptar(&self, id: AccountId) -> Result<Usuario> {
            self.usuarios_sin_aceptar.iter().find(|usuario| usuario.addres == id).cloned().ok_or(VotacionError::UsuarioSinAceptarNoEncontrado)
        }

        /// Obtiene un usuario por su id
        #[ink(message)]
        fn get_usuario(&self, id: AccountId) -> Result<Usuario> {
            self.usuarios.iter().find(|usuario| usuario.addres == id).cloned().ok_or(VotacionError::UsuarioNoEncontrado)
        }
    }

    impl EleccionSystemInk for Votacion {
        /// Dado un id: postula un candidato, si este esta postulado como votante, devolvera error
        #[ink(message)]
        fn postular_candidato(&mut self, id_eleccion: u32) -> Result<()> {
            let usuario_actual = self.env().caller();
            
            if self.get_usuario_sin_aceptar(usuario_actual).is_ok() {
                return Err(VotacionError::UsuarioNoAceptado);
            }

            if self.get_usuario(usuario_actual).is_err() {
                return Err(VotacionError::UsuarioNoEncontrado);
            }
            
            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.postular_candidato(usuario_actual, &Fecha::from_timestamp(timestamp))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Dado un id: postula un votante, si este esta postulado como candidato, devolvera error
        #[ink(message)]
        fn postular_votante(&mut self, id_eleccion: u32) -> Result<()> {
            let usuario_actual = self.env().caller();
            
            if self.get_usuario_sin_aceptar(usuario_actual).is_ok() {
                return Err(VotacionError::UsuarioNoAceptado);
            }

            if self.get_usuario(usuario_actual).is_err() {
                return Err(VotacionError::UsuarioNoEncontrado);
            }
            
            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.postular_votante(usuario_actual, &Fecha::from_timestamp(timestamp))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Dado un candidato postulado, es aceptado por el Admin
        #[ink(message)]
        fn agregar_candidato(&mut self, id_eleccion: u32, id_candidato: AccountId) -> Result<()> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            if self.get_usuario(id_candidato).is_err() {
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.agregar_candidato(id_candidato, &Fecha::from_timestamp(timestamp))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Dado un votante postulado, es aceptado por el Admin
        #[ink(message)]
        fn agregar_votante(&mut self, id_eleccion: u32, id_votante: AccountId) -> Result<()> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            if self.get_usuario(id_votante).is_err() {
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.agregar_votante(id_votante, &Fecha::from_timestamp(timestamp))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Vota por un candidato en una eleccion
        #[ink(message)]
        fn votar(&mut self, id_eleccion: u32, id_candidato: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if self.get_usuario(id_candidato).is_err() || self.get_usuario(caller).is_err(){
                return Err(VotacionError::UsuarioNoEncontrado);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.elecciones.get_mut(id_eleccion as usize) {
                eleccion.votar(&caller, &id_candidato, &Fecha::from_timestamp(timestamp))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Devuelve true si el usuario ya voto, false en cualquier otro caso
        #[ink(message)]
        fn ya_voto(&self, id_eleccion: u32, id_votante: AccountId) -> Result<bool> {
            if !self.caller_is_admin() {
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

        /// Devuelve si la elección ya inició
        #[ink(message)]
        fn get_iniciada(&self, id_eleccion: u32) -> Result<bool> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                Ok(eleccion.get_inicio(&Fecha::from_timestamp(timestamp)))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Devuelve si la elección ya finalizó
        #[ink(message)]
        fn get_finalizada(&self, id_eleccion: u32) -> Result<bool> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                Ok(eleccion.get_finalizada(&Fecha::from_timestamp(timestamp)))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

        /// Devuelve los votos de un candidato
        #[ink(message)]
        fn get_votos_candidato(&self, id_eleccion: u32, id_candidato: AccountId) -> Result<u32> {
            if !self.caller_is_admin() {
                return Err(VotacionError::NoEsAdmin);
            }

            let timestamp = self.env().block_timestamp();

            if let Some(eleccion) = self.get_eleccion(id_eleccion) {
                eleccion.get_votos_candidato(&id_candidato, &Fecha::from_timestamp(timestamp))
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }
    }

    impl ReportMessage for Votacion {
        /// Devuelve un vector con los id de los votantes aceptados
        #[ink(message)]
        fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<Vec<AccountId>> {
            if !self.caller_is_reporte() {
                return Err(VotacionError::SoloReportes);
            }

            let eleccion = self.get_eleccion(eleccion_id).ok_or(VotacionError::EleccionNoEncontrada)?;
            let id_votantes = eleccion.reporte_registro_votantes(); // -> AccountId de votantes aceptados y aprobados para esa eleccion
            
            Ok(id_votantes)
        }

        /// Devuelve un tuple que contiene la cantidad de votantes y la cantidad de votantes que votaron, en ese orden
        #[ink(message)]
        fn reporte_participacion(&self, eleccion_id: u32) -> Result<(u128, u128)> {
            if !self.caller_is_reporte() {
                return Err(VotacionError::SoloReportes);
            }
    
            let timestamp = self.env().block_timestamp();
            let eleccion = self.get_eleccion(eleccion_id).ok_or(VotacionError::EleccionNoEncontrada)?;
            
            eleccion.reporte_participacion(&Fecha::from_timestamp(timestamp))
        }

        /// Devuelve un vector que contiene para cada posicion el AccountID de un candidato y la cantidad de votos que obtuvo
        #[ink(message)]
        fn reporte_resultado(&self, eleccion_id: u32) -> Result<Vec<(AccountId, u32)>> {
            if !self.caller_is_reporte() {
                return Err(VotacionError::SoloReportes);
            }

            let timestamp = self.env().block_timestamp();

            let eleccion = self.get_eleccion(eleccion_id).ok_or(VotacionError::EleccionNoEncontrada)?;
            eleccion.reporte_resultado(&Fecha::from_timestamp(timestamp))
        }       
    }

    #[ink::trait_definition]
    pub trait ReportMessage {
        /// Reporte de registro de votantes para una elección específica
        #[ink(message)]
        fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<Vec<AccountId>>;
    
        /// Reporte de participación para una elección cerrada
        #[ink(message)]
        fn reporte_participacion(&self, eleccion_id: u32) -> Result<(u128, u128)>;
    
        /// Reporte de resultados finales de una elección cerrada
        #[ink(message)]
        fn reporte_resultado(&self, eleccion_id: u32) -> Result<Vec<(AccountId, u32)>>;
    }

    trait ReportMessageEleccion {
        /// Reporte de registro de votantes
        fn reporte_registro_votantes(&self) -> Vec<AccountId>;
    
        /// Reporte de participación
        fn reporte_participacion(&self, current_time: &Fecha) -> Result<(u128, u128)>;
    
        /// Reporte de resultados finales
        fn reporte_resultado(&self, current_time: &Fecha) -> Result<Vec<(AccountId, u32)>>;
    }

    #[cfg(test)]
    pub mod tests {
        use super::*;
        use ink::env::{test::{default_accounts, set_block_timestamp, set_caller}, DefaultEnvironment};

        /// Funcion auxiliar para los tests de reportes.
        pub fn default_with_data() -> Votacion {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            //Creo dos elecciones nuevas (admin)
            votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            //Agrego usuarios a la lista de usuarios_por_aceptar
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.crear_usuario("Alice".to_string(), "Cooper".to_string(), "EEUU".to_string(), "111".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Marley".to_string(), "Jamaica".to_string(), "222".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.charlie);
            votacion.crear_usuario("Charlie".to_string(), "Chaplin".to_string(), "Inglaterra".to_string(), "333".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.django);
            votacion.crear_usuario("Django".to_string(), "Unchained".to_string(), "EEUU".to_string(), "444".to_string(), 30).unwrap();
            //Acepto a los usuarios (admin)
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.alice).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            votacion.aceptar_usuario(accounts.charlie).unwrap();
            votacion.aceptar_usuario(accounts.django).unwrap();
            //Eleccion id = 0:
            //Postulo a alice y a bob como y candidatos a la eleccion
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(0).unwrap();
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_candidato(0).unwrap();
            //Postulo a charlie y a django como y votantes a la eleccion
            set_caller::<DefaultEnvironment>(accounts.charlie);
            votacion.postular_votante(0).unwrap();
            set_caller::<DefaultEnvironment>(accounts.django);
            votacion.postular_votante(0).unwrap();
            //Acepto a los votantes y candidatos postulados (admin)
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_candidato(0, accounts.alice).unwrap();
            votacion.agregar_candidato(0, accounts.bob).unwrap();
            votacion.agregar_votante(0, accounts.charlie).unwrap();
            votacion.agregar_votante(0, accounts.django).unwrap();

            //Eleccion id = 1
            //Postulo a alice como candidato a la eleccion    
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(1).unwrap();
            //Postulo a bob, charlie, y django como votantes de la eleccion
            set_caller::<DefaultEnvironment>(accounts.charlie);
            votacion.postular_votante(1).unwrap();
            set_caller::<DefaultEnvironment>(accounts.django);
            votacion.postular_votante(1).unwrap();
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(1).unwrap();
            //Acepto a los votantes y candidatos postulados (admin)
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_candidato(1, accounts.alice).unwrap();
            votacion.agregar_votante(1, accounts.bob).unwrap();
            votacion.agregar_votante(1, accounts.charlie).unwrap();
            votacion.agregar_votante(1, accounts.django).unwrap();
            //Realizo votos para eleccion 0 (alice = 2, bob = 0)
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.django);
            votacion.votar(0, accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.charlie);
            votacion.votar(0, accounts.alice).unwrap();
            //Realizo votos para eleccion 0 (alice = 2)
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.django);
            votacion.votar(1, accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.votar(1, accounts.alice).unwrap();

            votacion
        }
        

        #[ink::test]
        fn test_set_admin() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
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
                votantes_sin_aceptar: vec![],
                candidatos_sin_aceptar: vec![],
                votantes: votantes.clone(),
                candidatos: candidatos.clone(),
                votos: vec![],
                votantes_voto: vec![],
                fecha_inicio: Fecha::new(19, 6, 2024),
                fecha_fin: Fecha::new(20, 6, 2024),
            };

            assert_eq!(eleccion.get_id(), 0);
            assert!(eleccion.is_votante(&accounts.bob));
            assert!(!eleccion.is_votante(&accounts.charlie));
            assert!(eleccion.is_candidato(&accounts.django));
            assert!(!eleccion.is_candidato(&accounts.alice));
            assert_eq!(eleccion.get_fecha_inicio(), Fecha::new(19, 6, 2024));
            assert_eq!(eleccion.get_fecha_fin(), Fecha::new(20, 6, 2024));
        }

        // Tests de VotacionImpl
        #[ink::test]
        fn test_crear_eleccion() {
            let mut votacion = Votacion::default();
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2024);
            let id_eleccion = votacion.crear_eleccion(fecha_inicio.clone(), fecha_fin.clone()).unwrap();
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert_eq!(eleccion.get_id(), 0);
            assert_eq!(eleccion.get_fecha_inicio(), fecha_inicio);
            assert_eq!(eleccion.get_fecha_fin(), fecha_fin);
        }

        #[ink::test]
        fn test_crear_eleccion_error_no_admin() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::default();
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2024);
            set_caller::<DefaultEnvironment>(accounts.alice);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_crear_eleccion_error_fecha_inicio_mayor_que_fin() {
            let mut votacion = Votacion::default();
            let fecha_inicio = Fecha::new(1, 1, 2024);
            let fecha_fin = Fecha::new(31, 12, 2023);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::FechaInicioMayorQueFin));
        }

        #[ink::test]
        fn test_get_eleccion() {
            let mut votacion = Votacion::default();
            let eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            assert_eq!(votacion.get_eleccion(0).unwrap().get_id(), eleccion);
        }

        // Tests de UserManager
        #[ink::test]
        fn test_crear_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::default();
            let usuario = votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            assert_eq!(usuario.get_addres(), accounts.bob);
            assert_eq!(usuario.get_nombre(), "Juan".to_string());
            assert_eq!(usuario.get_apellido(), "Perez".to_string());
            assert_eq!(usuario.get_direccion(), "Calle Falsa 123".to_string());
            assert_eq!(usuario.get_dni(), "12345678".to_string());
            assert_eq!(usuario.get_edad(), 30);
        }

        #[ink::test]
        fn test_crear_usuario_error_usuario_no_aceptado() {
            let mut votacion = Votacion::default();
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            let usuario = votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_crear_usuario_error_usuario_ya_registrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
                set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::default();
            votacion.usuarios.push(Usuario::new(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30));
            let usuario = votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioYaRegistrado));
        }

        #[ink::test]
        fn test_aceptar_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::default();
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            assert_eq!(votacion.usuarios_sin_aceptar.len(), 0);
            assert_eq!(votacion.usuarios.len(), 1);
        }

        #[ink::test]
        fn test_aceptar_usuario_error_no_admin() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.charlie);
            let mut votacion = Votacion::default();
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let usuario = votacion.aceptar_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_aceptar_usuario_error_usuario_sin_aceptar_no_encontrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            let usuario = votacion.aceptar_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioSinAceptarNoEncontrado));
        }

        #[ink::test]
        fn test_get_usuario_sin_aceptar() {
            let accounts =
            default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::default();
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            assert_eq!(votacion.get_usuario_sin_aceptar(accounts.bob).unwrap().get_addres(), accounts.bob);
        }

        #[ink::test]
        fn test_get_usuario_sin_aceptar_error_usuario_sin_aceptar_no_encontrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let votacion = Votacion::default();
            let usuario = votacion.get_usuario_sin_aceptar(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioSinAceptarNoEncontrado));
        }

        #[ink::test]
        fn test_get_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.bob);
            let mut votacion = Votacion::default();
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();
            assert_eq!(votacion.get_usuario(accounts.bob).unwrap().get_addres(), accounts.bob);
        }

        #[ink::test]
        fn test_get_usuario_error_usuario_no_encontrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let votacion = Votacion::default();
            let usuario = votacion.get_usuario(accounts.bob);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoEncontrado));
        }

        //impl test de EleccionImpl
        #[ink::test]
        fn test_get_inicio_eleccion() {
            // Ya terminó
            let eleccion = Eleccion::new(0, Fecha::new(1, 1, 2023), Fecha::new(31, 12, 2023));
            assert!(eleccion.get_inicio(&Fecha::new(1, 1, 2024)));

            // Ya empezó y no terminó
            let eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            assert!(eleccion.get_inicio(&Fecha::new(15, 6, 2024)));

            // No empezó
            let eleccion = Eleccion::new(0, Fecha::new(1, 1, 2025), Fecha::new(31, 12, 2025));
            assert!(!eleccion.get_inicio(&Fecha::new(1, 1, 2024)));
        }

        #[test]
        fn test_get_finalizada_eleccion() {
            // Ya terminó
            let eleccion = Eleccion::new(0, Fecha::new(1, 1, 2023), Fecha::new(31, 12, 2023));
            assert!(eleccion.get_finalizada(&Fecha::new(1, 1, 2024)));

            // Ya empezó y no terminó
            let eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            assert!(!eleccion.get_finalizada(&Fecha::new(15, 6, 2024)));

            // No empezó
            let eleccion = Eleccion::new(0, Fecha::new(1, 1, 2025), Fecha::new(31, 12, 2025));
            assert!(!eleccion.get_inicio(&Fecha::new(1, 1, 2024)));
        }

        #[test]
        fn test_postular_candidato_eleccion() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert!(eleccion.is_postulado_candidato(&id_candidato));
            //Intento postularlo como votante
            assert_eq!(eleccion.postular_votante(id_candidato, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoCandidato));
            //Chequeo que no se haya postulado como votante
            assert!(!eleccion.is_postulado_votante(&id_candidato));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2024)), Err(VotacionError::EleccionYaIniciada));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_es_postulado_candidato() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoCandidato));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_es_postulado_votante() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoVotante));
        }

        #[test]
        fn test_postular_votante_eleccion() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_votante = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(id_votante, &Fecha::new(1, 1, 2023)).is_ok());
            assert!(eleccion.is_postulado_votante(&id_votante));
            //Intento postularlo como candidato
            assert_eq!(eleccion.postular_votante(id_votante, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoVotante));
            //Chequeo que no se haya postulado como candidato
            assert!(!eleccion.is_postulado_candidato(&id_votante));
        }

        #[test]
        fn test_postular_votante_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_votante(id_candidato, &Fecha::new(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }

        #[test]
        fn test_postular_votante_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_votante = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_votante(id_votante, &Fecha::new(1, 1, 2024)), Err(VotacionError::EleccionYaIniciada));
        }

        #[test]
        fn test_postular_votante_eleccion_error_es_postulado_votante() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_votante = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(id_votante, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.postular_votante(id_votante, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoVotante));
        }

        #[test]
        fn test_agregar_candidato_eleccion() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert!(eleccion.agregar_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert!(eleccion.is_candidato(&id_candidato));
        }

        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_no_postulado_candidato() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_candidato(id_candidato, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioNoPostuladoCandidato));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, &Fecha::new(15, 6, 2024)), Err(VotacionError::EleccionYaIniciada));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(1, 1, 2024));
            let id_candidato = AccountId::from([0x1; 32]);
            
            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, &Fecha::new(2, 1, 2024)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_candidato() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2025), Fecha::new(1, 1, 2025));
            let id_candidato = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert!(eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2024)).is_ok());
            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2024)), Err(VotacionError::UsuarioEsCandidato));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_votante() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2025), Fecha::new(1, 1, 2025));
            let id_candidato = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_votante(id_candidato, &Fecha::new(1, 1, 2024)).is_ok());
            assert!(eleccion.agregar_votante(id_candidato, &Fecha::new(31, 12, 2024)).is_ok());
            assert!(eleccion.postular_candidato(id_candidato, &Fecha::new(1, 1, 2024)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2024)), Err(VotacionError::UsuarioEsVotante));
        }

        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_votanteno_postulado() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2025), Fecha::new(1, 1, 2025));
            let id_candidato = AccountId::from([0x1; 32]);
    
            assert_eq!(eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2024)), Err(VotacionError::UsuarioNoPostuladoCandidato));
        }
    
        #[test]
        fn test_agregar_votante_eleccion() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2025), Fecha::new(1, 1, 2025));
            let votante_id = AccountId::from([0x2; 32]);
    
            assert!(eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2024)).is_ok());
            assert!(eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2024)).is_ok());
            assert!(eleccion.is_votante(&votante_id));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
    
            assert!(eleccion.postular_votante(votante_id, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, &Fecha::new(15, 6, 2024)), Err(VotacionError::EleccionYaIniciada));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(votante_id, &Fecha::new(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, &Fecha::new(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }

        #[test]
        fn test_agregar_votante_eleccion_error_usuario_no_postulado_votante() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_votante(votante_id, &Fecha::new(1, 1, 2023)), Err(VotacionError::UsuarioNoPostuladoVotante));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_usuario_es_candidato() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_candidato(votante_id, &Fecha::new(31, 12, 2023)).is_ok());
            assert!(eleccion.agregar_candidato(votante_id, &Fecha::new(31, 12, 2023)).is_ok());
            assert!(eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)), Err(VotacionError::UsuarioEsCandidato));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_usuario_es_votante() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).is_ok());
            assert!(eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).is_ok());
            assert!(eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)), Err(VotacionError::UsuarioEsVotante));
        }
    
        #[test]
        fn test_votar_eleccion() {
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));

            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();

            assert!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)).is_ok());
            assert_eq!(eleccion.get_votos_candidato(&id_candidato, &Fecha::new(1, 1, 2025)), Ok(1));
        }
    
        #[test]
        fn test_votar_eleccion_error_eleccion_no_iniciada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
    
            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(31, 12, 2023)), Err(VotacionError::EleccionNoIniciada));
        }
    
        #[test]
        fn test_votar_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
    
            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_votar_eleccion_error_usuario_no_es_votante_1() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);
            
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)), Err(VotacionError::UsuarioNoEsVotante));
        }

        #[test]
        fn test_votar_eleccion_error_usuario_no_es_votante_2() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);
            
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            //Postulo un al votante pero no lo acepta el admin
            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)), Err(VotacionError::UsuarioNoEsVotante));
        }
    
        #[test]
        fn test_votar_eleccion_error_usuario_no_es_candidato_1() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);

            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[test]
        fn test_votar_eleccion_error_usuario_no_es_candidato_2() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);

            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));
            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            //Postulo un candidato pero no lo acepta el admin
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[test]
        fn test_votar_eleccion_error_usuario_ya_voto() {
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));

            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();

            eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)).unwrap();
            
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)), Err(VotacionError::UsuarioYaVoto));
        }

        #[test]
        fn test_ya_voto_eleccion() {
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024));

            eleccion.postular_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, &Fecha::new(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, &Fecha::new(31, 12, 2023)).unwrap();

            assert!(!eleccion.ya_voto(&votante_id));
            eleccion.votar(&votante_id, &id_candidato, &Fecha::new(15, 6, 2024)).unwrap();
            assert!(eleccion.ya_voto(&votante_id));
        }



        // tests de EleccionSystemInk
        #[ink::test]
        fn test_postular_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(votacion.postular_candidato(id_eleccion).is_ok());
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert!(eleccion.is_postulado_candidato(&accounts.bob));
        }

        #[ink::test]
        fn test_postular_candidato_votacion_error_usuario_no_aceptado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            //Creo el usuario pero no es aceptado
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            //votacion.aceptar_usuario(accounts.bob).unwrap();

            assert_eq!(votacion.postular_candidato(id_eleccion), Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_postular_candidato_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            let resultado = votacion.postular_candidato(id_eleccion);
            assert_eq!(resultado, Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_agregar_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(votacion.postular_candidato(id_eleccion).is_ok());
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert!(votacion.agregar_candidato(id_eleccion, accounts.bob).is_ok());
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert!(eleccion.is_candidato(&accounts.bob));
        }

        #[ink::test]
        fn test_agregar_candidato_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(votacion.postular_candidato(id_eleccion).is_ok());
            let resultado = votacion.agregar_candidato(id_eleccion, accounts.bob);
            assert_eq!(resultado, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_postular_votante_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(votacion.postular_votante(id_eleccion).is_ok());
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert!(eleccion.is_postulado_votante(&accounts.bob));
        }

        #[ink::test]
        fn test_postular_votante_votacion_error_usuario_no_aceptado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            //Creo el usuario pero no es aceptado
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            //votacion.aceptar_usuario(accounts.bob).unwrap();

            assert_eq!(votacion.postular_votante(id_eleccion), Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_postular_votante_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            let resultado = votacion.postular_votante(id_eleccion);
            assert_eq!(resultado, Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_agregar_votante_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(votacion.postular_votante(id_eleccion).is_ok());

            set_caller::<DefaultEnvironment>(accounts.alice);
            assert!(votacion.agregar_votante(id_eleccion, accounts.bob).is_ok());
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert!(eleccion.is_votante(&accounts.bob));
        }

        #[ink::test]
        fn test_agregar_votante_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert!(votacion.postular_votante(id_eleccion).is_ok());
            let resultado = votacion.agregar_votante(id_eleccion, accounts.bob);
            assert_eq!(resultado, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_votar_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario("Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.bob);

            assert!(votacion.votar(id_eleccion, accounts.alice).is_ok());
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert!(eleccion.ya_voto(&accounts.bob));
        }

        #[ink::test]
        fn test_votar_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(votacion.votar(id_eleccion, accounts.alice), Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_ya_voto_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.crear_usuario("Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.votar(id_eleccion, accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            assert!(votacion.ya_voto(id_eleccion, accounts.bob).unwrap());
        }

        #[ink::test]
        fn test_ya_voto_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            assert_eq!(votacion.ya_voto(id_eleccion, accounts.alice), Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_ya_voto_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.ya_voto(id_eleccion, accounts.bob), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.crear_usuario("Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.votar(id_eleccion, accounts.alice).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2025).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.frank);
            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.alice).unwrap(), 1);
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.crear_usuario("Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.crear_usuario("Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.votar(id_eleccion, accounts.alice).unwrap();


            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2025).timestamp().unwrap());
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.alice), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_eleccion_no_encontrada() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let votacion = Votacion::default();
            assert_eq!(votacion.get_votos_candidato(0, accounts.bob), Err(VotacionError::EleccionNoEncontrada));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_eleccion_no_finalizada() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.crear_usuario("Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(5, 5, 2024).timestamp().unwrap());

            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.alice), Err(VotacionError::EleccionNoFinalizada));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_usuario_no_es_candidato() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(31, 12, 2023).timestamp().unwrap());
            
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.crear_usuario("Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.aceptar_usuario(accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(5, 5, 2025).timestamp().unwrap());

            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.bob), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[ink::test]
        fn test_get_iniciada_votacion() {
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            assert!(votacion.get_iniciada(id_eleccion).unwrap());
        }

        #[ink::test]
        fn test_get_iniciada_votacion_error_eleccion_no_encontrada() {
            let votacion = Votacion::default();
            assert_eq!(votacion.get_iniciada(0), Err(VotacionError::EleccionNoEncontrada));
        }

        #[ink::test]
        fn test_get_finalizada_votacion() {
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(Fecha::new(1, 1, 2024), Fecha::new(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2023).timestamp().unwrap());
            assert!(!votacion.get_finalizada(id_eleccion).unwrap());

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(15, 6, 2024).timestamp().unwrap());
            assert!(!votacion.get_finalizada(id_eleccion).unwrap());

            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2025).timestamp().unwrap());
            assert!(votacion.get_finalizada(id_eleccion).unwrap());
        }

        #[ink::test]
        fn test_get_finalizada_votacion_error_eleccion_no_encontrada() {
            let votacion = Votacion::default();
            assert_eq!(votacion.get_finalizada(0), Err(VotacionError::EleccionNoEncontrada));
        }

        #[ink::test]
        fn test_reporte_registro_votantes_eleccion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let eleccion = Eleccion {
                id: 0,
                fecha_inicio: Fecha::new(1, 1, 2024),
                fecha_fin: Fecha::new(31, 12, 2024),
                votantes_sin_aceptar: Vec::new(),
                votantes: vec![accounts.alice, accounts.bob, accounts.charlie],
                candidatos_sin_aceptar: Vec::new(),
                candidatos: Vec::new(),
                votos: Vec::new(),
                votantes_voto: Vec::new()
            };

            let votantes = eleccion.reporte_registro_votantes();
            assert_eq!(votantes.len(), 3);
            assert_eq!(votantes.get(0).unwrap(), &accounts.alice);
            assert_eq!(votantes.get(1).unwrap(), &accounts.bob);
            assert_eq!(votantes.get(2).unwrap(), &accounts.charlie);
        }

       #[ink::test]
        fn test_reporte_participacion_eleccion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let eleccion = Eleccion {
                id: 0,
                fecha_inicio: Fecha::new(1, 1, 2024),
                fecha_fin: Fecha::new(31, 12, 2024),
                votantes_sin_aceptar: Vec::new(),
                votantes: vec![accounts.alice, accounts.bob, accounts.charlie],
                candidatos_sin_aceptar: Vec::new(),
                candidatos: Vec::new(),
                votos: Vec::new(),
                votantes_voto: vec![accounts.alice, accounts.bob]
            };

            let participacion = eleccion.reporte_participacion(&Fecha::new(1, 1, 2025)).unwrap();
            assert_eq!(participacion.0, 3);
            assert_eq!(participacion.1, 2);
        }

        #[ink::test]
        fn test_reporte_participacion_eleccion_error_eleccion_no_finalizada() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let eleccion = Eleccion {
                id: 0,
                fecha_inicio: Fecha::new(1, 1, 2024),
                fecha_fin: Fecha::new(31, 12, 2024),
                votantes_sin_aceptar: Vec::new(),
                votantes: vec![accounts.alice, accounts.bob, accounts.charlie],
                candidatos_sin_aceptar: Vec::new(),
                candidatos: Vec::new(),
                votos: Vec::new(),
                votantes_voto: vec![accounts.alice, accounts.bob]
            };

            assert_eq!(eleccion.reporte_participacion(&Fecha::new(1, 1, 2024)), Err(VotacionError::EleccionNoFinalizada));
        }

        #[ink::test]
        fn test_reporte_resultado_eleccion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let eleccion = Eleccion {
                id: 0,
                fecha_inicio: Fecha::new(1, 1, 2024),
                fecha_fin: Fecha::new(31, 12, 2024),
                votantes_sin_aceptar: Vec::new(),
                votantes: vec![accounts.django, accounts.frank, accounts.charlie],
                candidatos_sin_aceptar: vec![accounts.alice, accounts.bob],
                candidatos: vec![accounts.alice, accounts.bob],
                votos: vec![(accounts.alice, 2), (accounts.bob, 0)],
                votantes_voto: vec![accounts.alice]
            };

            let mut resultado = eleccion.reporte_resultado(&Fecha::new(1, 1, 2025)).unwrap();
            assert_eq!(resultado.len(), 2);
            assert_eq!(resultado.pop().unwrap(), (accounts.bob, 0));
            assert_eq!(resultado.pop().unwrap(), (accounts.alice, 2));
        }

        #[ink::test]
        fn test_reporte_resultado_eleccion_error_eleccion_no_finalizada() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let eleccion = Eleccion {
                id: 0,
                fecha_inicio: Fecha::new(1, 1, 2024),
                fecha_fin: Fecha::new(31, 12, 2024),
                votantes_sin_aceptar: Vec::new(),
                votantes: vec![accounts.django, accounts.frank, accounts.charlie],
                candidatos_sin_aceptar: vec![accounts.alice, accounts.bob],
                candidatos: vec![accounts.alice, accounts.bob],
                votos: vec![(accounts.alice, 2), (accounts.bob, 0)],
                votantes_voto: vec![accounts.alice]
            };

            assert_eq!(eleccion.reporte_resultado(&Fecha::new(1, 1, 2024)), Err(VotacionError::EleccionNoFinalizada));
        }

        // tests de ReportMessage
        #[ink::test]
        fn test_reporte_registro_votantes() {
            let id_reporte = AccountId::from([0x10; 32]);
            let votacion = default_with_data(); //Eleccion de id = 0: 2 votantes aprobados/ eleccion de id = 1: 3 votantes aprobados
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(id_reporte);
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2026).timestamp().unwrap());
            let mut reporte = votacion.reporte_registro_votantes(0).unwrap();
            assert_eq!(reporte.len(), 2);
            assert_eq!(reporte.pop().unwrap(), accounts.django);
            assert_eq!(reporte.pop().unwrap(), accounts.charlie);
        }

        #[ink::test]
        fn test_reporte_participacion_votacion() {
            let id_reporte = AccountId::from([0x10; 32]);
            let votacion = default_with_data(); //Eleccion de id = 0: 2 votantes aprobados/ eleccion de id = 1: 3 votantes aprobados
            set_caller::<DefaultEnvironment>(id_reporte);
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2026).timestamp().unwrap());
            let reporte = votacion.reporte_participacion(0).unwrap();
            assert_eq!(reporte.0, 2);
            assert_eq!(reporte.1, 2);
        }

        #[ink::test]
        fn test_reporte_resultado() {
            let id_reporte = AccountId::from([0x10; 32]);
            let votacion = default_with_data(); //Eleccion de id = 0: Alice = 2 votos y Bob = 0 votos
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(id_reporte);
            set_block_timestamp::<DefaultEnvironment>(Fecha::new(1, 1, 2026).timestamp().unwrap());
            let mut reporte = votacion.reporte_resultado(0).unwrap();
            let primero = reporte.pop().unwrap();
            let segundo = reporte.pop().unwrap();
            assert_eq!(primero.0, accounts.bob);
            assert_eq!(primero.1, 0);
            assert_eq!(segundo.0, accounts.alice);
            assert_eq!(segundo.1, 2);
        }
    
        // tests de ReportMessageEleccion
    }
}

mod fecha {
    use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

    #[derive(Debug, Clone, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Fecha {
        day: u32,
        month: u32,
        year: i32,
        hour: u32,
        minute: u32,
        second: u32,
    }
    
    impl Fecha {
        /// Crea una fecha a partir de un timestamp en milisegundos
        pub fn from_timestamp(timestamp: u64) -> Self {
            let three_hours = 3 * 60 * 60;
            let datetime = DateTime::from_timestamp((timestamp / 1000) as i64 - three_hours, 0).unwrap();
            Fecha::new_with_time(
                datetime.day(), 
                datetime.month(), 
                datetime.year(), 
                datetime.hour(), // Le resto 3 horas para pasar de UTC a GMT-3
                datetime.minute(), 
                datetime.second(),
            )
        }
    
        pub fn new(day: u32, month: u32, year: i32) -> Self {
            Fecha::new_with_time(day, month, year, 0, 0, 0)
        }

        pub fn new_with_time(day: u32, month: u32, year: i32, hour: u32, minute: u32, second: u32) -> Self {
            Fecha {
                day,
                month,
                year,
                hour,
                minute,
                second
            }
        }
    
        pub fn es_fecha_valida(&self) -> bool {
            NaiveDate::from_ymd_opt(self.year, self.month, self.day).is_some() &&
                NaiveTime::from_hms_opt(self.hour, self.minute, self.second).is_some()
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

        /// Devuelve el timestamp en milisegundos
        pub fn timestamp(&self) -> Option<u64> {
            if !self.es_fecha_valida() {
                return None;
            }

            let date = NaiveDate::from_ymd_opt(self.year, self.month, self.day).expect("Fecha inválida");
            let time = NaiveTime::from_hms_opt(self.hour, self.minute, self.second).expect("Hora inválida");

            // Convertir la fecha a NaiveDateTime agregando tiempo
            let datetime = NaiveDateTime::new(date, time);
            // Obtener el timestamp
            let three_hours = 3 * 60 * 60;
            // Le resto 3 horas para pasar de GMT-3 a UTC
            let timestamp = (datetime.and_utc().timestamp() + three_hours) as u64;
            Some(timestamp * 1000)
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

            // Fecha inválida (hora fuera de rango)
            let fecha_invalida_hora = Fecha::new_with_time(15, 6, 2024, 24, 0, 0);
            assert!(!fecha_invalida_hora.es_fecha_valida());

            // Fecha inválida (minuto fuera de rango)
            let fecha_invalida_minuto = Fecha::new_with_time(15, 6, 2024, 23, 60, 0);
            assert!(!fecha_invalida_minuto.es_fecha_valida());

            // Fecha inválida (segundo fuera de rango)
            let fecha_invalida_segundo = Fecha::new_with_time(15, 6, 2024, 23, 59, 60);
            assert!(!fecha_invalida_segundo.es_fecha_valida());

            // Fecha válida (hora, minuto y segundo en rango)
            let fecha_valida_hora = Fecha::new_with_time(15, 6, 2024, 23, 59, 59);
            assert!(fecha_valida_hora.es_fecha_valida());

            // Fecha válida (hora, minuto y segundo en rango)
            let fecha_valida_hora = Fecha::new_with_time(15, 6, 2024, 0, 0, 0);
            assert!(fecha_valida_hora.es_fecha_valida());
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

        #[test]
        fn test_timestamp() {
            let fecha = Fecha::new_with_time(27, 6, 2024, 20, 9, 25);
            let timestamp = fecha.timestamp().unwrap();
            assert_eq!(timestamp, 1719529765000);
        }

        #[test]
        fn test_from_timestamp() {
            let fecha = Fecha::from_timestamp(1719529765000);
            assert_eq!(fecha, Fecha::new_with_time(27, 6, 2024, 20, 9, 25));
        }
    }
}

pub mod errors {
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
        UsuarioNoPostuladoCandidato,
        UsuarioNoPostuladoVotante,
        UsuarioEsPostuladoCandidato,
        UsuarioEsPostuladoVotante,
        SoloReportes,
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
                VotacionError::UsuarioNoPostuladoCandidato => write!(f, "Usuario no postulado como candidato"),
                VotacionError::UsuarioNoPostuladoVotante => write!(f, "Usuario no postulado como votante"),
                VotacionError::UsuarioEsPostuladoCandidato => write!(f, "Usuario postulado como candidato"),
                VotacionError::UsuarioEsPostuladoVotante => write!(f, "Usuario postulado como votante"),
                VotacionError::SoloReportes => write!(f, "Solo el contrato Reportes puede realizar esta operación"),
            }
        }
    }
}