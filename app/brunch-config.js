exports.config = {
	files: {
		javascripts: {
			joinTo: "js/app.js"
		},
		stylesheets: {
			joinTo: "css/app.css"
		}
	},
	// conventions: {
	//   assets: /^(static)/
	// },
	paths: {
		watched: [
			"assets", "src"//, "vendor", "static"
		],
		public: "./public"
	},
	plugins: {
		babel: {
			// ignore: [/vendor/]
		},
		elmBrunch: {
			elmFolder: ".",
			mainModules: ["src/main.elm"],
			outputFolder: "./assets/compiled-elm-app"
		},
		
	},
	modules: {
		autoRequire: {
			"./assets/js/app.js": ["js/main"]
		}
	}
}
