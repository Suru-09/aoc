import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer
import util.control.Breaks._

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/

class Instruction() {
    protected var cpuClock = 0
    protected var cycles = 0
    protected var isDone: Boolean = false
    protected var value: Int = 0

    def setCpuClock(currCpuClock: Int): Unit = {
        cpuClock = currCpuClock
    }

    def getCpuClock() : Int = cpuClock

    def isDone(currentCPUClock: Int): Boolean = {
        if(currentCPUClock - cpuClock == cycles) {
            isDone = true
        }
        isDone
    }

    def getIsDone(): Boolean = isDone
    def getValue(): Int = 0

    override def toString(): String = s"Instruction(c: $cycles, v: $value)"
}

class AddXInstruction(v: Int) extends Instruction {
    value = v
    cycles = 2

    override def getValue(): Int = value
}

class NopInstruction extends Instruction {
    cycles = 1
}

class SimpleCPU(startCount: Int, modulo: Int) {
    private var instrArray: ArrayBuffer[Instruction] = ArrayBuffer()
    private var crt: ArrayBuffer[Char] = ArrayBuffer.fill(240)('.')
    private var clock: Int = 1
    private var registerX: Int = 1
    private var signalsSum = 0

    def executeInstruction(instr: Instruction, empty: Boolean = false): Unit = {
        println(s"$instr starts executing")

        if (instrArray.length > 0 && instrArray.head.isDone(clock)) {
            registerX += instrArray.head.getValue()
            // if we do not set cpuClock again(the next instruction will consider that it already
            // has at least 1 clock cycle executed and therefore the next add will last only 1 cycle)
            instrArray.foreach(i => i.setCpuClock(clock))
            println(s"${instrArray.head} finished execution and x is: <$registerX>")
            // do the remove at end so we do not destroy indexes in the array(we access head for print etc.)
            instrArray.remove(0)  
        }

        //mind boggling that clock start at 1(but we must verifiy [clock - 1] because of 0-indexed arrays)
        if (List(registerX - 1, registerX, registerX + 1).exists(x => x == (clock - 1) % 40) == true) {
            crt(clock - 1) = '#'
        }
    
        if(clock == startCount || (clock - startCount) % modulo == 0) {
            signalsSum += registerX * clock
            println(s"Signal strength is: ${registerX * clock} and x = <$registerX>")
            println(s"Signal sum at clock: <$clock> is: ${signalsSum}")
        }

        if(!empty) {
            instr.setCpuClock(clock)
            instrArray = instrArray :+ instr
        }

        clock += 1
    }

    def finishExecution(): Unit = {
        while(instrArray.length > 0) {
            executeInstruction(Instruction(), true)
        }
    }

    def getSignalsSum() : Int = signalsSum
    def printCRT(): Unit = {
        println("\nCRT looks like this: ")
        var count = 1
        crt.foreach(ch => {
            print(ch)
            if( count % 40 == 0) {
                print("\n")
            }
            count += 1
        })
        println("\n")
    }
}


object RopeBridge {

    def matchInstruction(instrType: String): String = instrType match
        case "noop" => "nop"
        case "addx" => "add"
        case _ => "wrong"

    def solvePart1_Part2(file: String): Int = {
        var cpu = SimpleCPU(20, 40)
        for(line <- Source.fromFile(file).getLines) {
            var splitted = line.split(" ")
            var instrType = matchInstruction(splitted(0))
            if( instrType == "nop") {
                cpu.executeInstruction(NopInstruction())
            }
            else if(instrType == "add") {
                cpu.executeInstruction(AddXInstruction(splitted(1).toInt))
            } 
        }
        cpu.finishExecution()
        cpu.printCRT()
        cpu.getSignalsSum()
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(s"Requested signals sum is: ${solvePart1_Part2(args(0))}")
    }
}