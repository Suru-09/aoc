import scala.collection.mutable.Map
import scala.collection.mutable.Stack
import scala.io.Source
import util.control.Breaks._

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/
object SupplyStacks {

    def processLine(line: String): Map[Int, Char] = {
        var myMap = Map[Int, Char]()
        // ! The only reason I can use 4 and not get outOfBound from string is
        // ! because of the \n(newline) in the file, be careful at that!!
        val loadSize = 4
        var startSubstr = 0
        var endSubstr = loadSize
        var count = 0
        while(endSubstr <= line.length()) {
            var subString = line.substring(startSubstr, endSubstr)
            if (subString(1).isLetter) {
                myMap(count) = subString(1)
            }
            println(s"Substring: ${subString}")
            count += 1
            startSubstr += loadSize
            endSubstr += loadSize
        }
        var subString = line.substring(startSubstr, line.length())
        if (subString(1).isLetter) {
                myMap(count) = subString(1)
            }
            println(s"Substring: ${subString}")
        myMap
    }

    def processMoveline(line: String): (Int, Int, Int) = {
        var splittedLine = line.split("\\D+").filter(_.nonEmpty).toArray
        //splittedLine.foreach(println)
        (splittedLine(0).toInt - 1, splittedLine(1).toInt - 1, splittedLine(2).toInt - 1)
    }

    def move(cmd: (Int, Int, Int), myMap: Map[Int, Stack[Char]]): Map[Int, Stack[Char]] = {
        val movesNumber = cmd(0)
        val source = cmd(1)
        val destination = cmd(2)
        var modifiedMap = myMap

        for(cnt <- 0 to movesNumber) {
            val popped = modifiedMap(source).pop()
            modifiedMap(destination).push(popped)
        }
        modifiedMap
    }

    def moveInOrder(cmd: (Int, Int, Int), myMap: Map[Int, Stack[Char]]): Map[Int, Stack[Char]] = {
        val movesNumber = cmd(0)
        val source = cmd(1)
        val destination = cmd(2)
        var modifiedMap = myMap
        var tempStack = Stack[Char]()

        for(cnt <- 0 to movesNumber) {
            val popped = modifiedMap(source).pop()
            tempStack.push(popped)
        }
        
        tempStack.forall(elem => {
            modifiedMap(destination).push(elem)
            true
        })
        modifiedMap
    }

    def supply(file: String): String = {
        var moveFlag = false
        var stackMap =  Map[Int, Stack[Char]]()
        var currentMap = Map[Int, Char]()
        for(line <- Source.fromFile(file).getLines) {
            breakable {
                //println(s"Line: ${line}")
                if(line == "" || line(1) == '1') {
                    moveFlag = true
                    // ! before starting to move we must reverse every stack
                    // ! by pushing the elements in the stacks they ended up
                    // ! in reverse order
                    if (line == "") {
                        stackMap.foreach((key, stack) => {
                        stackMap.update(key, stackMap(key).reverse)
                        })
                        println(stackMap)
                    }   
                    break
                }
            
            
                if(!moveFlag) {
                    currentMap = processLine(line)
                    currentMap.foreach((key, value) => {
                        println(s"(Key, Value) -> ($key, $value)")
                        var foundKey = stackMap.get(key)
                        if (foundKey != None) {
                           stackMap.update(key, foundKey.get.push(value))
                        }
                        else {
                            stackMap(key) = Stack[Char](value)
                        }
                    })
                }
                else {
                    println(s"Line: ${line}")
                    stackMap = moveInOrder(processMoveline(line), stackMap)
                }
            }
        }
        println(stackMap)
        var result = ""
        stackMap.foreach((key, stack) => {
            result = result :+ stackMap(key).head
        })
        result
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(supply(args(0)))
    }
}