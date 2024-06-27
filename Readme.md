# Rust: Proyecto Final

## Miembros del grupo

- Vicente García Martí
- Maria Luisa Britez
- Julia Lunazzi
- Juan Cruz Pissaco

## Recordatorios

- `env().caller()` -> Devuelve el AccountId del que hizo la transacción
- La fecha del nodo es UTC, asi que para setear las fechas se debe modificar el tiempo actual a UTC que es igual a sumarle 3 horas
- Cada uno hace push a su branch, si funciona se hace push a la branch de development, si todo funciona bien se hara push a la branch master. Quedando el flujo de trabajo de la siguiente manera:
Branch personal -> development branch -> master branch.
De esta manera se evitara hacer deploys con errores
- **NO BUILDEAR NUNCA CON** `cargo build` **HACERLO SIMPRE CON** `cargo contract build`
