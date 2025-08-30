module Day3 exposing (..)

import Html exposing (pre, text)


main : Html.Html msg
main =
    pre [] [ text ("Part1: " ++ String.fromInt part1 ++ "\nPart2: " ++ String.fromInt part2) ]


part1 : Int
part1 =
    part1Helper inputs


part1Helper : String -> Int
part1Helper input =
    0


part2 : Int
part2 =
    part2Helper inputs


part2Helper : String -> Int
part2Helper input =
    0


inputs : String
inputs =