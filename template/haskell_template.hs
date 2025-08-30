import Data.Void
import System.IO
import Text.Read

main :: IO ()
main = do
  content <- readFile "../../../inputs/2019/01.txt"
  let input = inputParser content
  putStrLn $ "Part A: " ++ show (partA input)
  putStrLn $ "Part B: " ++ show (partB input)

------------ PARSER ------------
inputParser :: String -> Input
inputParser content =
  let linesOfInput = lines content
      parsed = mapM (readMaybe :: String -> Maybe Int) linesOfInput
   in case parsed of
        Just numbers -> numbers
        Nothing -> error "Invalid input: some lines could not be parsed as Int32"

------------ TYPES ------------
type Input = [Int]

type OutputA = Void

type OutputB = Void

------------ PART A ------------
partA :: Input -> OutputA
partA input = error "Not implemented yet!"

------------ PART B ------------
partB :: Input -> OutputB
partB input = error "Not implemented yet!"