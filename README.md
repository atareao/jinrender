# jinrender

`jinrender` es una sencilla aplicación de terminal que te permite renderizar variables de entorno en un archivo `jinja`. Por ejemplo, si tienes un archivo `template.jinja` con el siguiente contenido:

```jinja
Hola, {{ env.USERNAME }}!
¿Cómo estás?
```

Y tienes la variable de entorno `USERNAME` con el valor `atareao`, al ejecutar `jinrender -j template.jinja -o salida.txt` obtendrás:

```txt
Hola, atareao!
¿Cómo estás?
```

## ¿Para que lo utilizo?

Actualmente lo estoy utilizando principalmente para las portadas de los vídeos. En el archivo `template.jinja` tengo el contenido de la portada en formato `SVG`, realizado utilizando `inkscape` y en las variables de entorno tengo los datos que quiero que aparezcan en la portada. Por ejemplo, en el caso de un *podcast*, en las variables de entorno tengo el `título`, el número del episodio y el día en número de la publicación. De esta forma, al ejecutar `jinrender` obtengo la portada del episodio con los datos actualizados.
