import scala.collection.mutable.Map
import scala.io.Source

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/
object Rucksack {

    def stringToMap(str: String): Map[Char, Int] = {
        var myMap = Map[Char, Int]()
        val alphabetLength = 27
        str.foreach(ch => {
            if (ch.isUpper) {
                // alphatet length == 27 -> mapping {A ... Z} -> {27 ... 52}
                myMap(ch) = ch.toInt - 'A'.toInt + alphabetLength
                //println(s"Value: ${ch} -> ${ch.toInt - 'A'.toInt + alphabetLength}")
            }
            else {
                // adding 1 in order to map {a ... z } -> {1 ... 26} instead of {0 ... 25}
                myMap(ch) = ch.toInt - 'a'.toInt + 1
                //println(s"Value: ${ch} -> ${ch.toInt - 'a'.toInt + 1}")
            }
        })
        myMap
    }

    def calculateLine(line: String): Int = {
        var res = 0
        val len = line.length()
        val firstHalf = line.substring(0, len / 2)
        val secondHalf = line.substring(len / 2, len)
        
        val firstMap = stringToMap(firstHalf)
        val secondMap = stringToMap(secondHalf)

        // make intersection of the maps and create a new map based on their intersection
        val resultMap = firstMap.keySet.intersect(secondMap.keySet).map(k => (k, firstMap(k))).toMap
        //resultMap.foreach(println)
        resultMap.foreach((key, value) => res += value)
        res
    }

    def findSecurityItem(elfGroup: Array[String]): Int = {
        var frequencyArray: Array[Map[Char, Int]] = Array()
        elfGroup.foreach(elf => {
            frequencyArray = frequencyArray :+ stringToMap(elf)
        })
        var firstIntersection = frequencyArray(0).keySet.intersect(frequencyArray(1).keySet).map(k => (k, frequencyArray(0)(k))).toMap
        var finalIntersection = firstIntersection.keySet.intersect(frequencyArray(2).keySet).map(k => (k, firstIntersection(k))).toMap
        finalIntersection.foreach((key, value) => {
            println(s"Common Key: ${key} and value: ${value}")
        })
        finalIntersection.values.sum
    }

    def rucsack(file: String): Int = {
        var result = 0
        var count = 0
        var elfGroup = Array[String]()
        for(line <- Source.fromFile(file).getLines) {
            println(line)
            elfGroup = elfGroup :+ line
            count += 1

            if (count == 3) {
                result += findSecurityItem(elfGroup)
                elfGroup = Array[String]()
                count = 0
            }
            //result += calculateLine(line)
        }
        result
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(rucsack(args(0)))
    }
}