import Data.Int (Int32)
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
      parsed = mapM (readMaybe :: String -> Maybe Int32) linesOfInput
   in case parsed of
        Just numbers -> numbers
        Nothing -> error "Invalid input: some lines could not be parsed as Int32"

------------ TYPES ------------
type Input = [Int32]

type OutputA = Int32

type OutputB = Int32

------------ PART A ------------
partA :: Input -> OutputA
partA input = sum $ map (\x -> x `div` 3 - 2) input

------------ PART B ------------
partB :: Input -> OutputB
partB input = sum $ map fuleForModuleWithFule input
  where
    fuleForModuleWithFule x = go (x `div` 3 - 2)
    go fuel
      | fuel <= 0 = 0
      | otherwise = fuel + go (fuel `div` 3 - 2)