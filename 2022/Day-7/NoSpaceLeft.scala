import scala.collection.mutable.Map
import scala.collection.mutable.ArrayDeque
import scala.collection.mutable.Stack
import scala.io.Source
import scala.collection.mutable.ArrayBuffer

/*
    DISCLAIMER: The following solution works for PART 2 of the problem!
*/

trait SystemObject {
    def getName(): String
    def isDirectory(): Boolean
    def getSize(): Int
}

trait DirectoryOptions extends SystemObject {
    def addFile(objects: SystemObject*):Unit
}

class File(n: String, s: Int) extends SystemObject {
    private val name: String = n
    private val size: Int = s

    def getSize(): Int = size
    def getName(): String = name
    def isDirectory(): Boolean = false
    
    override def toString(): String = s"- ${name} (file, size = $size)\n"
}

class Directory(n: String) extends DirectoryOptions {
    private val name: String = n
    private var systemObject = Array[SystemObject]()
    private var prevDir: Directory = null
    def getName(): String = name
    def addFile(objects: SystemObject*): Unit = {
        objects.foreach(obj => {
            systemObject = systemObject :+ obj
        })
    }

    def setPrevious(prev: Directory): Unit = {
        prevDir = prev
    }
    def getPrevious(): Directory = prevDir
    
    override def toString(): String = {
        var result = String("")
        if (name == "/") {
            result += s"- ${name} (dir)\n"
        }
        systemObject.foreach(obj => {
            result += nSpaces(depthToRoot())
            if (obj.isDirectory()) {
                result += s"- ${obj.getName()} (dir)\n"
            }
            result += obj.toString()
        })
        result
    }

    def getSize(): Int = {
        var sum: Int = 0
        systemObject.foreach(obj => {
            sum += obj.getSize()
        })
        sum
    }

    def getDirectories(): Array[Directory] = {
        var arr = Array[Directory]()
        systemObject.foreach(obj => {
            if (obj.isDirectory())
                arr = arr :+ obj.asInstanceOf[Directory]
        })
        arr
    }

    def depthToRoot(): Int = {
        var dir: Directory = getPrevious()
        var count = 1
        while(dir != null) {
            count += 1
            dir = dir.getPrevious()
        }
        count
    }

    def nSpaces(n: Int): String = {
        var spaces = ""
        for(i <- 0 to n - 1) {
            spaces = spaces + "  "
        }
        spaces
    }

    def isDirectory(): Boolean = true

    def getDir(dirName: String): Directory = {
        var result: Directory = null
        systemObject.foreach(obj => {
            if(obj.isDirectory() && obj.getName() == dirName) {
                result = obj.asInstanceOf[Directory]
            }
        })
        if(result != null) result else Directory("empty")
    }

}

object NoSpaceLeft {

    def matchCommand(line: String): String = line.substring(0, 4) match
        case "$ cd" => "cd"
        case "$ ls" => "ls"
        case _ => "other"
    
    def cdArg(line: String): String = line.split(" ").last

    def isDirectory(line: String): Boolean = line(0).isLetter match
        case true => true
        case false => false
    
    def findSmallestDirectoryToDelete(head: Directory, dir: Directory, minSize: Int): Int = {
        var minSize = Int.MaxValue
        var dirList = dir.getDirectories()

        dirList.foreach( directory => {
            if (70_000_000 - head.getSize() + directory.getSize() >= 30_000_000) {
                minSize = minSize.min(directory.getSize())
            }
            minSize = minSize.min(findSmallestDirectoryToDelete(head, directory, minSize))
        })
        minSize
    }

    def countEligibleDirectories(dir: Directory): Int = {
        var sum = 0
        var dirList = dir.getDirectories()

        dirList.foreach( directory => {
            var size =  directory.getSize()
            if( size <= 100_000) {
                //println(s"Directory: ${directory.getName()} and size: ($size)")
                sum += size
            }
            sum += countEligibleDirectories(directory)
        })
        sum
    }

    def findSize(file: String): Int = {
        var head = Directory("/")
        var currentDir = head
        var lsFlag = false
        for(line <- Source.fromFile(file).getLines) {
            if (!lsFlag && matchCommand(line) == "ls") {
                lsFlag = true
            }

            if(matchCommand(line) == "cd") {
                lsFlag = false
                if( cdArg(line) == "..") {
                    //println(s"I am going back: ${currentDir.getPrevious().getName()}")
                    currentDir = currentDir.getPrevious()
                }
                else if(cdArg(line) == "/"){
                    currentDir = head
                }
                else {
                    //println(s"I am going forward: ${currentDir.getDir(cdArg(line)).getName()}")
                    currentDir = currentDir.getDir(cdArg(line))
                }
            }

            if(lsFlag && matchCommand(line) == "other") {
                if(isDirectory(line)) {
                    var dirName = line.split(" ")(1)
                    //println(s"Dir name: ${dirName}")
                    var dir = Directory(dirName)
                    dir.setPrevious(currentDir)
                    currentDir.addFile(dir)
                }
                else {
                    var fileArr = line.split(" ")
                    var file = File(fileArr(1), fileArr(0).toInt)
                    currentDir.addFile(file)
                }
            }
        }
        println(head)
        println(findSmallestDirectoryToDelete(head, head, 0))
        countEligibleDirectories(head)
    }

    def main(args: Array[String]): Unit = {
        if (args.length == 0) {
            throw new Exception("You should provide the input file!")
        }
        println(findSize(args(0)))
    }
}