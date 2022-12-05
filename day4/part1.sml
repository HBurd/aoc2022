fun readFile (filename: string) = let
  val file = TextIO.openIn filename
  fun readLines file = 
    case TextIO.inputLine file of
         SOME line => line :: readLines file
       | NONE => []
in
  readLines file
end;

val lines = readFile "input.txt";

val size = String.size;
val sub = String.sub;
val extract = String.extract;
val toInt = Int.fromString;

fun chop (c: char) (x: string) = let
  fun chopRecurse (x: string, c: char, i: int) =
    if i >= size x then ("0", "0")
    else if sub (x, i) = c then (extract (x, 0, SOME i), extract (x, i+1, NONE))
    else chopRecurse (x, c, i + 1)
in
  chopRecurse (x, c, 0)
end;

fun pairMap f (a, b) = (f a, f b);

fun parseRange s = (pairMap toInt) (chop #"-" s);
fun parseLine s = (pairMap parseRange) (chop #"," s);

fun removeInvalid (((SOME a, SOME b),(SOME c, SOME d)) :: rest) = ((a, b), (c,d)) :: removeInvalid rest
  | removeInvalid (_ :: rest) = removeInvalid rest
  | removeInvalid [] = [];

val ranges = removeInvalid (map parseLine lines);

fun contains (a, b) (c, d) = c >= a andalso d <= b;
fun doesOverlap (r1, r2) = contains r1 r2 orelse contains r2 r1;
fun doesPartiallyOverlap ((a, b), (c, d)) = b >= c andalso a <= d;

fun count f (a :: rest) =
  if f a then 1 + count f rest
  else count f rest
  | count f [] = 0;

val overlapCount = count doesOverlap ranges;
val partialOverlapCount = count doesPartiallyOverlap ranges;
