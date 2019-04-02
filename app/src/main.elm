-- https://guide.elm-lang.org/webapps/navigation.html
-- https://guide.elm-lang.org/webapps/url_parsing.html


module Main exposing (main)

import Browser
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (..)
import Url
import Url.Parser



-- MAIN


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        }



-- MODEL


type alias Model =
    { key : Nav.Key
    , route : Route
    }


type alias Flags =
    ()


type alias Post =
    { id : Int
    , title : String
    , subtitle : String
    }


type Route
    = Home
    | Posts
    | NotFound


init : Flags -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init _ url key =
    ( { key = key
      , route = toRoute url
      }
    , Cmd.none
    )



-- UPDATE


type Msg
    = LinkClicked Browser.UrlRequest
    | UrlChanged Url.Url


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        LinkClicked urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model, Nav.pushUrl model.key (Url.toString url) )

                Browser.External href ->
                    ( model, Nav.load href )

        UrlChanged url ->
            ( { model | route = toRoute url }, Cmd.none )


toRoute : Url.Url -> Route
toRoute url =
    Maybe.withDefault NotFound (Url.Parser.parse routeParser url)


routeParser : Url.Parser.Parser (Route -> a) a
routeParser =
    Url.Parser.oneOf
        [ Url.Parser.map Home Url.Parser.top
        , Url.Parser.map Posts (Url.Parser.s "posts")
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    { title = "Elm SPA Example"
    , body = viewBody model
    }


viewBody : Model -> List (Html Msg)
viewBody model =
    [ viewHeader model
    , viewContent model
    , viewFooter model
    ]


viewHeader : Model -> Html Msg
viewHeader model =
    div []
        [ text "-- HEADER --"
        , ul []
            [ li [] [ a [ href "/" ] [ text "home" ] ]
            , li [] [ viewLink "posts" ]
            ]
        ]


viewLink : String -> Html msg
viewLink path =
    a [ href ("/" ++ path) ] [ text path ]


viewContent : Model -> Html Msg
viewContent model =
    div []
        [ text "-- CONTENT: "
        , viewPage model
        ]


viewPage : Model -> Html Msg
viewPage model =
    case model.route of
        Home ->
            homePage model

        Posts ->
            postsPage model

        NotFound ->
            notFoundPage model


viewFooter : Model -> Html Msg
viewFooter model =
    div []
        [ text "-- FOOTER --"
        ]


homePage : Model -> Html Msg
homePage model =
    text "Home Page"


postsPage : Model -> Html Msg
postsPage model =
    text "Posts Page"


notFoundPage : Model -> Html Msg
notFoundPage model =
    text "Not Found Page"
