# Experimentación
En este directorio se encuentra la experimentación de la tesis que se puede encontrar en el reporte. La estructura de directorios es la siguiente:

* ```templates```: contiene los templates de programas de Noir que son citados en el reporte
* ```programas_generados```: son los programas generados a partir de los templates. Para generar este directorio y su contenido hay que ejecutar ```python generar_programas.py```.
* ```backends```: son los binarios asociados a los distintos backends mencionados en el reporte. Al ser binarios es posible (y altamente probable) que no funcionen en un entorno distinto al que fueron compilados. Para recompilarlos hay que seguir los pasos en la ```implementación```. Para generar las distintas variantes es necesario modificar la configuración del circuito en ```implementacion/plonky2-backend/src/circuit_translation/mod.rs```. 
* ```nargo_versions```: son los binarios con los cuales deben compilarse los programas de ```programas_generados```. 
* ```mediciones```: contiene tanto las mediciones de tiempos y de tamaños como los gráficos generados a partir de ellas.
* ```generar_programas.py``` sirve para generar los programas de Noir a partir de los templates, asi como ```borrar_programas.py``` sirve para borrarlos.
* ```experimentacion.ipynb``` es una notebook donde se irán ejecutando todos los pasos para correr los experimentos. Puede ser necesario comentar/descomentar ciertas lineas o secciones dependiendo lo que se necesite. 