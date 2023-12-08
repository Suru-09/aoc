
import scala.io.Source;

/*
    DISCLAIMER: Solution works for PART 2, not PART 1
*/

object CalorieCounter {

    def countCalories(fileName: String): Int = {
        var calorieArray = Array[Int]()
        var sum = 0
        for (line <- Source.fromFile(fileName).getLines) {
            if (line != "") {
                sum += line.toInt
            }
            else {
                calorieArray = calorieArray :+ sum
                sum = 0;
            }
        }
        calorieArray = calorieArray.sortWith(_ > _)
        calorieArray(0) + calorieArray(1) + calorieArray(2)
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide a file input in order to use the program!")
        }
        println(countCalories(args(0)))
    }

}