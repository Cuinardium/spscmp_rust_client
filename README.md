TPE-Protos
===========

Un servidor pop3 con capacidades de monitoreo y configuracion en runtime. 





## Ubicacion de archivos relevantes

- El codigo fuente del servidor se encuentra en la carpeta `server/src/`.
- El codigo fuente del cliente de monitoreo se encuentra en la carpeta `monitor/src/`.
- Los archivos con las reglas de construccion son los archivos `makefile`
- El informe y los archivos RFC pertinentes se encuentran en la carpeta `docs/`
## Construccion
Para generar los ejectuables basta con correr `make all` en la raiz del proyecto.
## Testeo

Para correr los testeos desarrollados basta con correr `make test`.
## Correr

Los ejecutables a correr son `pop3_monitor` y `pop3_server`. Estos se encuentran en la raiz del proyecto.

### Correr servidor POP3

El ejecutable del servidor pop3 no recibe ningun argumento. La informacion de los usuarios se encuentra en el directorio `server/data/`.

### Correr cliente de monitor

El ejecutable del cliente del monitor recibe como argumentos los siguientes

- <direccion_del_servidor>: especifica la direccion ip del servidor de monitoreo.
- <token_de_autenticacion>: especifica el token de autenticacion acordado con el servidor. De ser un token erroneo el servidor de monitoreo no respondera a las consultas.
- <comando>: especifica el comando a correr del protocolo de monitoreo.
- [argumento_de_comando]: para los comandos que precisan de argumentos.

El orden en el que se reciben estos argumentos es el siguiente :
- `./pop3_monitor <direccion_del_servidor> <token_de_autenticacion> <comando> [argumento_de_comando] [argumento_de_comando]`

Los posibles comandos son:
- -ADD_USER: Implementa el comando ADDUSER del protocolo, su primer argumento es el nombre de usuario y su segundo es la contraseña.
- -DELETE_USER: Implementa el comando DELUSER del protocolo, su unico argumento es el nombre de usuario
- -CHANGE_PASSWORD: Implementa el comando UPDATEPASS del protocolo, su primer argumento es el nombre de usuario y su segundo es la contraseña nueva.
- -CHANGE_USERNAME: Implementa el comando UPDATENAME del protocolo, su primer argumento es el nombre de usuario y su segundo es el nombre de usuario nuevo.
- -LIST: Implementa el comando LIST del protocolo, no tiene argumentos.
- -METRICS: Implementa el comando METRICS del protocolo, no tiene argumentos.
- -LOGS: Implementa el comando LOGS del protocolo, no tiene argumentos.
- -SET_MAX_USERS: Implementa el comando MAXUSERS del protocolo, su unico argumento es la cantidad maxima de usuarios.
- -SET_MAX_CONNS: Implementa el comando MAXCONNS del protocolo, su unico argumento es la cantidad maxima de conexiones.



## Autores

- [@cuinardium](https://github.com/Cuinardium)
- [@jBafico](https://github.com/jBafico)
- [@francobosetti](https://github.com/francobosetti)
- [@ImNotGone](https://github.com/ImNotGone)

