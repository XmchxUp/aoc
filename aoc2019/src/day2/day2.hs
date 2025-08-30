import Data.Void
import System.IO
import Text.Read
import Prelude

split :: Char -> String -> [String]
split _ "" = []
split c xs = start : split c (drop 1 rest)
  where
    start = takeWhile (/= c) xs
    rest = dropWhile (/= c) xs

main :: IO ()
main = do
  content <- readFile "../../../inputs/2019/02.txt"
  let input = inputParser content
  putStrLn $ "Part A: " ++ show (partA input)
  putStrLn $ "Part B: " ++ show (partB input)

------------ PARSER ------------
inputParser :: String -> Input
inputParser content =
  let linesOfInput = lines content
   in map read (split ',' $ head linesOfInput) :: [Int]

------------ TYPES ------------
type Input = [Int]

type OutputA = Int

type OutputB = Int

------------ PART A ------------
partA :: Input -> OutputA
partA input =
  let mem1 = take 1 input ++ [12] ++ [2] ++ drop 3 input
   in head (execute 0 mem1)

------------ PART B ------------
partB :: Input -> OutputB
partB input =
  let target = 19690720
      (noun, verb) =
        head
          [ (n, v) | n <- [0 .. 99], v <- [0 .. 99], let mem = take 1 input ++ [n, v] ++ drop 3 input, head (execute 0 mem) == target
          ]
   in 100 * noun + verb

execute :: Int -> [Int] -> [Int]
execute ip mem
  | op == 99 = mem
  | otherwise = execute (ip + 4) newMem
  where
    op = mem !! ip
    pos1 = mem !! (ip + 1)
    pos2 = mem !! (ip + 2)
    posOut = mem !! (ip + 3)
    val1 = mem !! pos1
    val2 = mem !! pos2
    result
      | op == 1 = val1 + val2
      | op == 2 = val1 * val2
      | otherwise = error "Unknown opcode"
    newMem = take posOut mem ++ [result] ++ drop (posOut + 1) mem