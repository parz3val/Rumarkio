module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)



-- MAIN


main : Program () Model Msg
main =
    Browser.sandbox
        { init = init
        , update = update
        , view = view
        }



-- Model


type alias Model =
    { name : String
    }


init : Model
init =
    Model ""



-- UDPATE


type Msg
    = Name String


update : Msg -> Model -> Model
update msg model =
    case msg of
        Name name ->
            { model | name = "Hello " ++ name }



-- View


view : Model -> Html Msg
view model =
    div []
        [ viewInput "text" "Name" "" Name
        , hr []
            []
        , div
            []
            [ text model.name ]
        ]


viewInput : String -> String -> String -> (String -> Msg) -> Html Msg
viewInput title p v toMsg =
    input [ type_ title, placeholder p, value v, onInput toMsg ] []
