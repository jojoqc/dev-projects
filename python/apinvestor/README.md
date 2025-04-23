# API Rest para obtener informacion de inversiones en diferentes mercados

Este proyecto consiste en una API Rest que proporciona informacion sobre inversiones en diferentes mercados. La API se comunica con diferentes fuentes de datos para obtener informacion actualizada sobre los instrumentos financieros que se negocian en diferentes mercados.

La API esta escrita en Python utilizando el framework FastAPI y utiliza una base de datos SQLite para almacenar la informacion. La documentacion de la API se puede encontrar en la seccion [Documentacion](#documentacion).

## Documentacion

La documentacion de la API se puede encontrar en la siguiente direccion: [http://localhost:5000/redoc](http://localhost:5000/redoc) o alternativamente [http://localhost:5000/docs](http://localhost:5000/docs) y [http://localhost:5000/docs](http://localhost:5000/openapi.json). Alli se puede encontrar informacion detallada sobre los endpoints disponibles, los parametros que se pueden pasar y los formatos de respuesta esperados.

## Instalacion

Para utilizar la API, se ejecuta el siguiente comando: `fastapi dev src/main.py`. Es necesario tener Python 3.9 o superior instalado previamente.

## Desarrollo

El desarrollo de la API se realizara en varias etapas. 
La primera etapa consistira en implementar los endpoints para obtener informacion de diferentes instrumentos financieros en diferentes mercados. 
La segunda etapa sera la implementacion de los endpoints para obtener informacion de los indices bursatiles de diferentes mercados. 
La tercera etapa sera la implementacion de los endpoints para obtener informacion de los instrumentos financieros emitidos por el estado de diferentes paises.

## Hitos

* Implementar los endpoints para obtener informacion de los instrumentos financieros en diferentes mercados.
* Implementar los endpoints para obtener informacion de los indices bursatiles de diferentes mercados.
* Implementar los endpoints para obtener informacion de los instrumentos financieros emitidos por el estado de diferentes paises.
* Implementar un sistema de autenticacion y autorizacion para los usuarios.
* Implementar un sistema de notificaciones para los usuarios.
* Implementar un sistema de busqueda de instrumentos financieros.
