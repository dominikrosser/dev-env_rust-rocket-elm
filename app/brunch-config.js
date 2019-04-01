exports.config = {
	files: {
		javascripts: {
			joinTo: "js/app1.js"
		},
		stylesheets: {
			joinTo: "css/app1.css"
		}
	},
	//conventions: {
	// assets: /^(static)/
	//},
	paths: {
		watched: [
			"assets", "src"//, "vendor", "static"
		],
		public: "./public"
	},
	plugins: {
		babel: {
			ignore: [/vendor/]
		},
		elmBrunch: {
			elmFolder: ".",
			mainModules: ["src/main.elm"],
			outputFolder: "./assets/js",
			outputFile: "compiled-elm-app.js",
			optimize: true,
			makeParameters: ['']
		},
		
	},
	modules: {
		//autoRequire: {
		//	"./assets/js/app.js": ["js/main"]
		//}
	}
}
