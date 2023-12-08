import scala.io.Source

/*
    DISCLAIMER: Solution works for PART 2, not PART 1
*/

object RockPaperScissors {

    def splitLine(line: String): Array[String] = {
        line.split(" ")
    }

    // this functions return 1 if the 2 given pieces are equivalent, 0 otherwise
    def areEqual(x: String, a: String): Boolean = {
        var result = false
        if (x == "X" && a == "A") {
            result = true
        }
        else if (x == "Y" && a == "B") {
            result = true
        }
        else if (x == "Z" && a == "C") {
            result = true
        }
        result
    }

    // returns score of the first adversary based on the input( a vs x).
    def playerScore(x: String, a: String): Int = {
        var score = 0
        println(x)
        println(a)
        if (areEqual(x, a)) {
            score = 3
        }
        else if( x == "X" && a == "C") {
            score = 6
        }
        else if( x == "Y" && a == "A") {
            score = 6
        }
        else if ( x == "Z" && a == "B") {
            score = 6
        }
        score
    }

    // returns points for the choice(as an int), -1 otherwise.
    def piecesScore(option: String): Int = option match
        case "X" => 1
        case "Y" => 2
        case "Z" => 3
        case _ => -1
    

    // function created only for formatting the output
    def matchOpponentPiece(piece: String): String = piece match
        case "A" => "Rock"
        case "B" => "Paper"
        case "C" => "Scissors"
        case _ => "Wrong"
    
    // function created only for formatting the output
    def matchMyChoice(piece: String): String = piece match
        case "X" => "Lose"
        case "Y" => "Draw"
        case "Z" => "Win"
        case _ => "Wrong"
    
    // for the input a return something that beats that piece
    def win(a: String): String = a match
        case "A" => "Y"
        case "B" => "Z"
        case "C" => "X"
        case _ => "Wrong"
    
    // for the input a return something that loses to that piece
    def lose(a: String): String = a match
        case "A" => "Z"
        case "B" => "X"
        case "C" => "Y"
        case _ => "Wrong"
    
    // for the input a return equivalent piece
    def draw(a: String): String = a match
        case "A" => "X"
        case "B" => "Y"
        case "C" => "Z"
        case _ => "Wrong"
    
    
    def gettingHelp(help: String, a: String): Int = help match
        case "X" => playerScore(lose(a), a) + piecesScore(lose(a))
        case "Y" => playerScore(draw(a), a) + piecesScore(draw(a))
        case "Z" => playerScore(win(a), a) + piecesScore(win(a))
        case _ => -1
    


    def game(file: String): Int = {
        var result = 0
        for(line <- Source.fromFile(file).getLines) {
            val strategy = splitLine(line.toString())
            
            val roundScore = gettingHelp(strategy(1), strategy(0))
            println(s"Opponent Chooses: ${matchOpponentPiece(strategy(0))} and I should: ${matchMyChoice(strategy(1))}")
            result += roundScore
            println(s"Resulting in me obtainting: $roundScore points")
        }
        result
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(game(args(0)))
    }
}