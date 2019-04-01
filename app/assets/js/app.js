const elmDiv = document.querySelector('#elm-container');

if (elmDiv) {
	Elm.Main.init({ node: elmDiv });
} else {
	Debug.log ('elm container div does not exist');
}
