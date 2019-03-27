import Elm from '../compiled-elm-app/main';// FIXME can't include

const elmDiv = document.querySelector('#elm-container');

if (elmDiv) {
	Elm.Main.embed(elmDiv);
} else {
	Debug.log ('elm container div does not exist');
}
