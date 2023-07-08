Spscmp Rust  Client
===========

Un cliente de spscmp(Simple POP3 Server Configuration and Monitoring Protocol) escrito en rust. El protocolo en cuestion es utilizado para la configuracion y monitoreo de un servidor POP3, su especificacion se encuentra en el archivo `docs/RFC_SPSCMP.txt`. Este proyecto fue creado para practicar el uso del lenguaje, la implementacion original del cliente del protocolo se encuentra en el siguiente repositiorio: [TP-Protos](https://github.com/ImNotGone/TP-Protos).






## Construccion
Para generar los ejectuables basta con correr `cargo build`.
## Testeo

Para correr los testeos desarrollados basta con correr `cargo test`.
## Correr

El ejecutable a correr es `spscmp_client`. Este se encuentra en la carpeta `target/debug/` o `target/release/` dependiendo con que flags se compilo el ejecutable.

- Uso: `spscmp_client [opciones] <comando>`
- Opciones:
    - `-p <puerto>` Puerto en el que escuchar. De no ser especificado se utiliza el puerto 8889
    - `-t <token>` Token de autenticación. De no ser especificado se utiliza el valor definido por la variable de entorno `SPSCMP_AUTH_TOKEN`,      si no se ha especificado un token en particular y la variable de entorno no esta definida el programa falla.
- Comandos:
    - `adduser <nombre> <contraseña>` Agregar un nuevo usuario
    - `deluser <nombre>` Eliminar un usuario
    - `updatepass <nombre> <contraseña>` Actualizar la contraseña de un usuario
    - `updatename <nombre> <nuevonombre>` Actualizar el nombre de un usuario
    - `listusers` Listar todos los usuarios
    - `metrics` Obtener métricas del servidor
    - `logs` Obtener registros del servidor
    - `maxusers <num>` Establecer el número máximo de usuarios
    - `maxconns <num>` Establecer el número máximo de conexiones


