import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import util.control.Breaks._

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/

class Monkey(n: Long, op: Array[String], t: Long, it: ArrayBuffer[Long], tCond: Long, fCond: Long) {
    private val number: Long = n
    private val operation: Array[String] = op
    private val testValue: Long = t
    private var items: ArrayBuffer[Long] = it
    private val trueCond: Long = tCond
    private val falseCond: Long = fCond
    private var thrownItems = 0

    def formatItems(): String = {
        var string = ""
        var firstFlag = true
        items.foreach(item => {
            if(firstFlag) {
                firstFlag = false
                string = string + item.toString()
            }
            else {
                string = string + ", " + item.toString()
            }
        })
        string
    }
    override def toString(): String = s"Monkey(n: $number, op: ${op(0)}, t: $testValue, items(${formatItems()})"

    def cOp(op: String, itemValue: Long): Long = {
        var result: Long = -1
        var isNumberFlag = true
        op.foreach(ch => {
            if(!ch.isDigit) {
                isNumberFlag = false
            }
        })

        if(isNumberFlag) {
            result = op.toLong
        } else if (op == "old")
            result = itemValue
        result
    }

    def addItem(item: Long): Unit = {
        items += item
    }

    def matchOperation(): Long = operation(1) match
        case "*" => 
            cOp(operation(0), items.head) * cOp(operation(2), items.head) 
        case "/" => 
            cOp(operation(0), items.head) / cOp(operation(2), items.head) 
        case "+" =>
            cOp(operation(0), items.head) + cOp(operation(2), items.head) 
        case "-" =>
            cOp(operation(0), items.head) - cOp(operation(2), items.head) 
        case _ => -1

    def getItems(): ArrayBuffer[Long] = items
    def getThrownItems(): Long = thrownItems

    def throwItems(monkeyDivisor: Long): (Long, Long) = {
        var worryLevel: Long = matchOperation()
        var monkeyNo: Long = -1
        monkeyNo = if(worryLevel % testValue == 0) trueCond else falseCond
        // comment this in order to solve PART 1
        worryLevel %= monkeyDivisor
        //println(s"MonkeyDivide: $monkeyDivisor")

        thrownItems += 1
        items.remove(0)
        (worryLevel, monkeyNo)
    }
}

class MonkeyGame(r: Long) {
    private val rounds: Long = r
    private var monkeys: ArrayBuffer[Monkey] = ArrayBuffer()
    private var monkeyDivisor: Long = 1

    def readMonkeyNo(line: String): Long = {
        line.split(" ")(1).split(":")(0).toLong
    }

    def readItems(line: String): ArrayBuffer[Long] = {
        var numbersArr = line.split(":")(1).split(",")
        var intArr: ArrayBuffer[Long] = ArrayBuffer()
        numbersArr.foreach(str => intArr += str.trim().toLong)
        intArr
    }

    def readOperation(line: String): Array[String] = {
        // arr -> contains the (right side )
        // e.g. new = old * 3 --> arr contains old, *, 3
        var arr = line.split("=")(1).split(" ")
        arr.foreach(str => str.trim())
        arr = arr.filter(str => str != "")
        arr
    }

    def readTest(line: String): Long = {
        line.split(" ").last.trim().toLong
    }

    def readMonkeys(file: String): Unit = {
        var countLine: Long = 0
        var number: Long = -1
        var testTemp: Long = -1
        var opTemp: Array[String] = Array()
        var itemsTemp: ArrayBuffer[Long] = ArrayBuffer()
        var trueCond: Long = -1
        var falseCond: Long = -1
        var monkeyLen: Long = 6   // 6 lines  in the file for a monkey

        for(line <- Source.fromFile(file).getLines) {
            if(!line.isEmpty()) {
                (countLine % monkeyLen) match
                    case 0 => number = readMonkeyNo(line)
                    case 1 => itemsTemp = readItems(line)
                    case 2 => opTemp = readOperation(line)
                    case 3 => testTemp = readTest(line)
                    case 4 => trueCond = readTest(line)
                    case 5 => {
                        falseCond = readTest(line)
                        monkeyDivisor *= testTemp
                        var monkey = Monkey(number, opTemp, testTemp, itemsTemp, trueCond, falseCond)
                        monkeys += monkey
                        //println(monkey)
                    }
                    case _ => number = -1
                countLine += 1
            }
        }
    }

    def calculateMonkeyBussiness(): Long = {
        val values = monkeys.sortWith(_.getThrownItems() > _.getThrownItems()).take(2)
        values(0).getThrownItems() * values(1).getThrownItems()
    }

    def executeOneRound(): Unit = {
        monkeys.foreach(monkey => {
            while(monkey.getItems().length > 0) {
                val t = monkey.throwItems(monkeyDivisor)
                monkeys(t(1).toInt).addItem(t(0))
            }   
        })
    }

    def executeRounds(): Unit = {
        for(i <- 1 to rounds.toInt) {
            executeOneRound()
            println(s"After round <$i> the monkeys are holding the following items: ")
            printMonkeys()
        }
    }

    def printMonkeys(): Unit = {
        monkeys.foreach(println)
    }
}

object MonkeyMiddle {

    def solvePart1(file: String): Long = {
        var monkeyGame = MonkeyGame(10000)
        monkeyGame.readMonkeys(file)
        monkeyGame.executeRounds()
        monkeyGame.calculateMonkeyBussiness()
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(solvePart1(args(0)))
    }
}