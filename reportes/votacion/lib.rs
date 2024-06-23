#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]
pub use self::votacion::{
    Votacion,
    VotacionRef,
};
pub use self::errors::VotacionError;

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
        votantes_sin_aceptar: Vec<AccountId>,
        candidatos_sin_aceptar:Vec<AccountId>,
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
        /// Devuelve true si el id esta postulado como candidato
        fn is_postulado_candidato(&self, id: &AccountId) -> bool;
        /// Devuelve true si el id esta postulado como votante
        fn is_postulado_votante(&self, id: &AccountId) -> bool;
        /// Dado un id , devuelve true si esta registrado como votante en la eleccion
        fn is_votante(&self, id: &AccountId) -> bool;
        /// Dado un id , devuelve true si esta registrado como candidato en la eleccion
        fn is_candidato(&self, id: &AccountId) -> bool;
        //Devuelve la cantidad de votantes
        fn get_cant_votantes(&self) -> u32;
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
        /// Postula un candidato, si este esta postulado como votante, devolvera error
        fn postular_candidato(&mut self, id_candidato: AccountId, current_time: Timestamp) -> Result<()>;
        /// Postula un votante, si este esta postulado como candidato, devolvera error
        fn postular_votante(&mut self, id_votante: AccountId, current_time: Timestamp) -> Result<()>;
        /// Dado un candidato postulado, es aceptado por el Admin
        fn agregar_candidato(&mut self, id_candidato: AccountId, current_time: Timestamp) -> Result<()>;
        /// Dado un votante postulado, es aceptado por el Admin
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
        /// Devuelve los votos de todos los candidatos, almacenados por id
        fn get_votos(&self, current_time: Timestamp) -> Result< Vec<(AccountId, u32)> >;
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
        reporte: AccountId,
        elecciones: Vec<Eleccion>,
        usuarios: Vec<Usuario>,
        usuarios_sin_aceptar: Vec<Usuario>,
    }

    impl Eleccion {
        pub fn new(id: u32, fecha_inicio: Timestamp, fecha_fin: Timestamp) -> Self {
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
        fn postular_votante(&mut self, id_votante: AccountId, current_time: Timestamp) -> Result<()> {
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
            //Aniado votante postulado
            self.votantes_sin_aceptar.push(id_votante);
            Ok(())
        }

        fn postular_candidato(&mut self, id_candidato: AccountId, current_time: Timestamp) -> Result<()> {
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
            //Aniado candidato postulado
            self.candidatos_sin_aceptar.push(id_candidato);
            Ok(())
        }

        fn agregar_candidato(&mut self, id_candidato: AccountId, current_time: Timestamp) -> Result<()> {    
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if self.get_inicio(current_time) {
                return Err(VotacionError::EleccionYaIniciada);
            }

            if !self.is_postulado_candidato(&id_candidato) {
                return Err(VotacionError::UsuarioNoPostuladoCandidato);
            }
            //No es necesario, debido a que postularse a votante/candidato es excluyente ->
            /*if self.is_postulado_votante(&id_candidato) {
                return Err(VotacionError::UsuarioEsPostuladoVotante);
            }*/

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

        fn agregar_votante(&mut self, id_votante: AccountId, current_time: Timestamp) -> Result<()> {
            if self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionYaFinalizada);
            }

            if self.get_inicio(current_time) {
                return Err(VotacionError::EleccionYaIniciada);
            }

            if !self.is_postulado_votante(&id_votante) {
                return Err(VotacionError::UsuarioNoPostuladoVotante);
            }
            //No es necesario, debido a que postularse a votante/candidato es excluyente ->
            /*if self.is_postulado_candidato(&id_candidato) {
                return Err(VotacionError::UsuarioEsPostuladoVotante);
            }*/

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

        fn get_votos(&self, current_time: Timestamp)  -> Result< Vec<(AccountId, u32)> > {
            if !self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionNoFinalizada);
            }
            Ok(self.votos.clone())
        }
    }

    impl GettersEleccion for Eleccion {
        fn get_id(&self) -> u32 {
            self.id
        }

        fn is_postulado_candidato(&self, id: &AccountId) -> bool {
            self.candidatos_sin_aceptar.iter().any(|candidato| candidato == id)
        }

        fn is_postulado_votante(&self, id: &AccountId) -> bool {
            self.votantes_sin_aceptar.iter().any(|votante| votante == id)
        }

        fn is_votante(&self, id: &AccountId) -> bool {
            self.votantes.iter().any(|votante| votante == id)
        }

        fn is_candidato(&self, id: &AccountId) -> bool {
            self.candidatos.iter().any(|candidato| candidato == id)
        }

        fn get_cant_votantes(&self) -> u32 {
            self.votantes.len() as u32
        }
        fn get_fecha_inicio(&self) -> Timestamp {
            self.fecha_inicio
        }

        fn get_fecha_fin(&self) -> Timestamp {
            self.fecha_fin
        }
    }

    //TODO: Hacer test de esta implementacion
    impl ReportMessageEleccion for Eleccion {
        fn reporte_registro_votantes(&self) -> u32 {
            self.get_cant_votantes()
        }

        fn reporte_participacion(&self, current_time: Timestamp) -> Result<(u32, u128)> {
            if !self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionNoFinalizada);
            }

            let num_votantes = self.votantes.len() as u128;
            let num_votantes_voto = self.votantes_voto.len() as u128;

            if num_votantes == 0 {
                return Ok((0, 0));
            }

            let participacion = (num_votantes_voto / num_votantes ) * 100;

            Ok((num_votantes_voto as u32, participacion))
        }

        fn reporte_resultado(&self, current_time: Timestamp) -> Result<Vec<(AccountId, u32)>> {
            if !self.get_finalizada(current_time) {
                return Err(VotacionError::EleccionNoFinalizada);
            }
    
            let mut resultados = self.votos.clone();
            resultados.sort_by_key(|(_, voto)| *voto);
    
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

    impl Votacion {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                admin: Self::env().caller(),
                reporte: AccountId::from([0x10; 32]),
                elecciones: Vec::new(),
                usuarios: Vec::new(),
                usuarios_sin_aceptar: Vec::new()
            }
        }

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
                eleccion.postular_candidato(usuario_actual, timestamp)
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

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
                eleccion.postular_votante(usuario_actual, timestamp)
            } else {
                Err(VotacionError::EleccionNoEncontrada)
            }
        }

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

    //TODO: Hacer test de esta implementacion
    impl ReportMessage for Votacion {
        #[ink(message)]
        fn is_reporte(&self, caller: AccountId) -> bool {
            self.reporte == caller
        }

        #[ink(message)]
        fn reporte_registro_votantes(&self, eleccion_id: u32) -> Result<u32> {
            if self.is_reporte(self.env().caller()) {
                return Err(VotacionError::SoloReportes);
            }

            let eleccion = self.get_eleccion(eleccion_id).ok_or(VotacionError::EleccionNoEncontrada)?;
            
            Ok(eleccion.reporte_registro_votantes())
        }

        #[ink(message)]
        fn reporte_participacion(&self, eleccion_id: u32) -> Result<(u32, u128)> {
            if self.is_reporte(self.env().caller()) {
                return Err(VotacionError::SoloReportes);
            }
    
            let eleccion = self.get_eleccion(eleccion_id).ok_or(VotacionError::EleccionNoEncontrada)?;
            
            eleccion.reporte_participacion(self.env().block_timestamp())
        }

        #[ink(message)]
        fn reporte_resultado(&self, eleccion_id: u32) -> Result<Vec<(AccountId, u32)>> {
            if self.is_reporte(self.env().caller()) {
                return Err(VotacionError::SoloReportes);
            }

            let eleccion = self.get_eleccion(eleccion_id).ok_or(VotacionError::EleccionNoEncontrada)?;
            eleccion.reporte_resultado(self.env().block_timestamp())
        }       
    }

    #[ink::trait_definition]
    pub trait ReportMessage {
        /// Devuelve true si el caller es el reporte, false en cualquier otro caso
        #[ink(message)]
        fn is_reporte(&self, caller: AccountId) -> bool;

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

    trait ReportMessageEleccion {
        /// Reporte de registro de votantes
        fn reporte_registro_votantes(&self) -> u32;
    
        /// Reporte de participación
        fn reporte_participacion(&self, current_time: Timestamp) -> Result<(u32, u128)>;
    
        /// Reporte de resultados finales
        fn reporte_resultado(&self, current_time: Timestamp) -> Result<Vec<(AccountId, u32)>>;
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
            let mut votacion = Votacion::default();
            let fecha_inicio = create_date(1, 1, 2024);
            let fecha_fin = create_date(31, 12, 2024);
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
            let fecha_inicio = create_date(1, 1, 2024);
            let fecha_fin = create_date(31, 12, 2024);
            set_caller::<DefaultEnvironment>(accounts.alice);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_crear_eleccion_error_fecha_inicio_mayor_que_fin() {
            let mut votacion = Votacion::default();
            let fecha_inicio = create_date(1, 1, 2024);
            let fecha_fin = create_date(31, 12, 2023);
            let eleccion = votacion.crear_eleccion(fecha_inicio, fecha_fin);
            assert_eq!(eleccion, Err(VotacionError::FechaInicioMayorQueFin));
        }

        #[ink::test]
        fn test_get_eleccion() {
            let mut votacion = Votacion::default();
            let eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            assert_eq!(votacion.get_eleccion(0).unwrap().get_id(), eleccion);
        }

        // Tests de UserManager
        #[ink::test]
        fn test_crear_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
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
            let mut votacion = Votacion::default();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_crear_usuario_error_usuario_ya_registrado() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            votacion.usuarios.push(Usuario::new(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30));
            let usuario = votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30);
            assert_eq!(usuario, Err(VotacionError::UsuarioYaRegistrado));
        }

        #[ink::test]
        fn test_aceptar_usuario() {
            let accounts =
                default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
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
            let mut votacion = Votacion::default();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let mut votacion = Votacion::default();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let mut votacion = Votacion::default();
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
        fn test_postular_candidato_eleccion() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert!(eleccion.is_postulado_candidato(&id_candidato));
            //Intento postularlo como votante
            assert_eq!(eleccion.postular_votante(id_candidato, create_date(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoCandidato));
            //Chequeo que no se haya postulado como votante
            assert!(!eleccion.is_postulado_votante(&id_candidato));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2024)), Err(VotacionError::EleccionYaIniciada));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_es_postulado_candidato() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoCandidato));
        }

        #[test]
        fn test_postular_candidato_eleccion_error_es_postulado_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoVotante));
        }

        #[test]
        fn test_postular_votante_eleccion() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_votante = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(id_votante, create_date(1, 1, 2023)).is_ok());
            assert!(eleccion.is_postulado_votante(&id_votante));
            //Intento postularlo como candidato
            assert_eq!(eleccion.postular_votante(id_votante, create_date(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoVotante));
            //Chequeo que no se haya postulado como candidato
            assert!(!eleccion.is_postulado_candidato(&id_votante));
        }

        #[test]
        fn test_postular_votante_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_votante(id_candidato, create_date(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }

        #[test]
        fn test_postular_votante_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_votante = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.postular_votante(id_votante, create_date(1, 1, 2024)), Err(VotacionError::EleccionYaIniciada));
        }

        #[test]
        fn test_postular_votante_eleccion_error_es_postulado_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_votante = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(id_votante, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.postular_votante(id_votante, create_date(1, 1, 2023)), Err(VotacionError::UsuarioEsPostuladoVotante));
        }

        #[test]
        fn test_agregar_candidato_eleccion() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert!(eleccion.agregar_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert!(eleccion.is_candidato(&id_candidato));
        }

        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_no_postulado_candidato() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_candidato(id_candidato, create_date(1, 1, 2023)), Err(VotacionError::UsuarioNoPostuladoCandidato));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let id_candidato = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, create_date(15, 6, 2024)), Err(VotacionError::EleccionYaIniciada));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(1, 1, 2024));
            let id_candidato = AccountId::from([0x1; 32]);
            
            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, create_date(2, 1, 2024)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_candidato() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let id_candidato = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert!(eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2024)).is_ok());
            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2024)), Err(VotacionError::UsuarioEsCandidato));
        }
    
        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let id_candidato = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_votante(id_candidato, create_date(1, 1, 2024)).is_ok());
            assert!(eleccion.agregar_votante(id_candidato, create_date(31, 12, 2024)).is_ok());
            assert!(eleccion.postular_candidato(id_candidato, create_date(1, 1, 2024)).is_ok());
            assert_eq!(eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2024)), Err(VotacionError::UsuarioEsVotante));
        }

        #[test]
        fn test_agregar_candidato_eleccion_error_usuario_es_votanteno_postulado() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let id_candidato = AccountId::from([0x1; 32]);
    
            assert_eq!(eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2024)), Err(VotacionError::UsuarioNoPostuladoCandidato));
        }
    
        #[test]
        fn test_agregar_votante_eleccion() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2025), create_date(1, 1, 2025));
            let votante_id = AccountId::from([0x2; 32]);
    
            assert!(eleccion.postular_votante(votante_id, create_date(31, 12, 2024)).is_ok());
            assert!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2024)).is_ok());
            assert!(eleccion.is_votante(&votante_id));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_eleccion_ya_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
    
            assert!(eleccion.postular_votante(votante_id, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(15, 6, 2024)), Err(VotacionError::EleccionYaIniciada));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);

            assert!(eleccion.postular_votante(votante_id, create_date(1, 1, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }

        #[test]
        fn test_agregar_votante_eleccion_error_usuario_no_postulado_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);

            assert_eq!(eleccion.agregar_votante(votante_id, create_date(1, 1, 2023)), Err(VotacionError::UsuarioNoPostuladoVotante));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_usuario_es_candidato() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_candidato(votante_id, create_date(31, 12, 2023)).is_ok());
            assert!(eleccion.agregar_candidato(votante_id, create_date(31, 12, 2023)).is_ok());
            assert!(eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)), Err(VotacionError::UsuarioEsCandidato));
        }
    
        #[test]
        fn test_agregar_votante_eleccion_error_usuario_es_votante() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x1; 32]);
    
            assert!(eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).is_ok());
            assert!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).is_ok());
            assert!(eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).is_ok());
            assert_eq!(eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)), Err(VotacionError::UsuarioEsVotante));
        }
    
        #[test]
        fn test_votar_eleccion() {
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));

            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();

            assert!(eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)).is_ok());
            assert_eq!(eleccion.get_votos_candidato(&id_candidato, create_date(1, 1, 2025)), Ok(1));
        }
    
        #[test]
        fn test_votar_eleccion_error_eleccion_no_iniciada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
    
            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(31, 12, 2023)), Err(VotacionError::EleccionNoIniciada));
        }
    
        #[test]
        fn test_votar_eleccion_error_eleccion_ya_finalizada() {
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
    
            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(1, 1, 2025)), Err(VotacionError::EleccionYaFinalizada));
        }
    
        #[test]
        fn test_votar_eleccion_error_usuario_no_es_votante_1() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);
            
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)), Err(VotacionError::UsuarioNoEsVotante));
        }

        #[test]
        fn test_votar_eleccion_error_usuario_no_es_votante_2() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);
            
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            //Postulo un al votante pero no lo acepta el admin
            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)), Err(VotacionError::UsuarioNoEsVotante));
        }
    
        #[test]
        fn test_votar_eleccion_error_usuario_no_es_candidato_1() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);

            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[test]
        fn test_votar_eleccion_error_usuario_no_es_candidato_2() {
            let id_candidato = AccountId::from([0x1; 32]);
            let votante_id = AccountId::from([0x2; 32]);

            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));
            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            //Postulo un candidato pero no lo acepta el admin
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
    
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[test]
        fn test_votar_eleccion_error_usuario_ya_voto() {
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));

            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();

            eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)).unwrap();
            
            assert_eq!(eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)), Err(VotacionError::UsuarioYaVoto));
        }

        #[test]
        fn test_ya_voto_eleccion() {
            let votante_id = AccountId::from([0x2; 32]);
            let id_candidato = AccountId::from([0x1; 32]);
            let mut eleccion = Eleccion::new(0, create_date(1, 1, 2024), create_date(31, 12, 2024));

            eleccion.postular_votante(votante_id, create_date(31, 12, 2023)).unwrap();
            eleccion.postular_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_candidato(id_candidato, create_date(31, 12, 2023)).unwrap();
            eleccion.agregar_votante(votante_id, create_date(31, 12, 2023)).unwrap();

            assert!(!eleccion.ya_voto(&votante_id));
            eleccion.votar(&votante_id, &id_candidato, create_date(15, 6, 2024)).unwrap();
            assert!(eleccion.ya_voto(&votante_id));
        }



        // tests de EleccionSystemInk
        #[ink::test]
        fn test_postular_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            //Creo el usuario pero no es aceptado
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            //votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(votacion.postular_candidato(id_eleccion), Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_postular_candidato_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            let resultado = votacion.postular_candidato(id_eleccion);
            assert_eq!(resultado, Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_agregar_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            //Creo el usuario pero no es aceptado
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            //votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(votacion.postular_votante(id_eleccion), Err(VotacionError::UsuarioNoAceptado));
        }

        #[ink::test]
        fn test_postular_votante_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            let resultado = votacion.postular_votante(id_eleccion);
            assert_eq!(resultado, Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_agregar_votante_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Juan".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();
            
            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));

            assert!(votacion.votar(id_eleccion, accounts.bob, accounts.alice).is_ok());
            let eleccion = votacion.get_eleccion(id_eleccion).unwrap();
            assert!(eleccion.ya_voto(&accounts.bob));
        }

        #[ink::test]
        fn test_votar_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.votar(id_eleccion, accounts.bob, accounts.alice), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_votar_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.votar(id_eleccion, accounts.bob, accounts.alice), Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_ya_voto_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            votacion.votar(id_eleccion, accounts.bob, accounts.alice).unwrap();
            assert!(votacion.ya_voto(id_eleccion, accounts.bob).unwrap());
        }

        #[ink::test]
        fn test_ya_voto_votacion_error_usuario_no_encontrado() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.alice);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            assert_eq!(votacion.ya_voto(id_eleccion, accounts.alice), Err(VotacionError::UsuarioNoEncontrado));
        }

        #[ink::test]
        fn test_ya_voto_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(votacion.ya_voto(id_eleccion, accounts.bob), Err(VotacionError::NoEsAdmin));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            
            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            votacion.votar(id_eleccion, accounts.bob, accounts.alice).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(1, 1, 2025));
            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.alice).unwrap(), 1);
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_no_es_admin() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.bob, "Bob".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345678".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.bob).unwrap();

            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();

            set_caller::<DefaultEnvironment>(accounts.bob);
            votacion.postular_votante(id_eleccion).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_votante(id_eleccion, accounts.bob).unwrap();
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
            votacion.votar(id_eleccion, accounts.bob, accounts.alice).unwrap();


            set_block_timestamp::<DefaultEnvironment>(create_date(1, 1, 2025));
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(5, 5, 2024));

            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.alice), Err(VotacionError::EleccionNoFinalizada));
        }

        #[ink::test]
        fn test_get_votos_candidato_votacion_error_usuario_no_es_candidato() {
            let accounts = default_accounts::<DefaultEnvironment>();
            set_caller::<DefaultEnvironment>(accounts.frank);
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(31, 12, 2023));
            
            votacion.crear_usuario(accounts.alice, "Alice".to_string(), "Perez".to_string(), "Calle Falsa 123".to_string(), "12345679".to_string(), 30).unwrap();
            votacion.aceptar_usuario(accounts.alice).unwrap();
            set_caller::<DefaultEnvironment>(accounts.alice);
            votacion.postular_candidato(id_eleccion).unwrap();

            set_caller::<DefaultEnvironment>(accounts.frank);
            votacion.agregar_candidato(id_eleccion, accounts.alice).unwrap();

            set_block_timestamp::<DefaultEnvironment>(create_date(5, 5, 2025));

            assert_eq!(votacion.get_votos_candidato(id_eleccion, accounts.bob), Err(VotacionError::UsuarioNoEsCandidato));
        }

        #[ink::test]
        fn test_get_iniciada_votacion() {
            let mut votacion = Votacion::default();
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(15, 6, 2024));
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
            let id_eleccion = votacion.crear_eleccion(create_date(1, 1, 2024), create_date(31, 12, 2024)).unwrap();
            set_block_timestamp::<DefaultEnvironment>(create_date(1, 1, 2025));
            assert!(votacion.get_finalizada(id_eleccion).unwrap());
        }

        #[ink::test]
        fn test_get_finalizada_votacion_error_eleccion_no_encontrada() {
            let votacion = Votacion::default();
            assert_eq!(votacion.get_finalizada(0), Err(VotacionError::EleccionNoEncontrada));
        }

        // tests de ReportMessage
    
    
        // tests de ReportMessageEleccion
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