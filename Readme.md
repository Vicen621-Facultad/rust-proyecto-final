# Rust: Proyecto Final

## Miembros del grupo

- Vicente García Martí
- Maria Luisa Britez
- Julia Lunazzi
- Juan Cruz Pissaco

## Recordatorios

- `env().caller()` -> Devuelve el AccountId del que hizo la transacción
- Los timestamps estan en milisegundos, asi que para poner la fecha de la eleccion se debe usar la siguiente [pagina](https://www.epochconverter.com/) y copiar el tiempo en milisegundos
- Cada uno hace push a su branch, si funciona se hace push a la branch de development, si todo funciona bien se hara push a la branch master. Quedando el flujo de trabajo de la siguiente manera:
Branch personal -> development branch -> master branch.
De esta manera se evitara hacer deploys con errores
- **NO BUILDEAR NUNCA CON** `cargo build` **HACERLO SIMPRE CON** `cargo contract build`
