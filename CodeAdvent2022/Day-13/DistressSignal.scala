import scala.io.Source

object DistressSignal {

    def readInput(filename: String): Seq[Seq[String]] = {
        Source.fromFile(filename).getLines.toList.filter(_ != "").grouped(2).toList
    }

    def splitList(list: String): Seq[String] = {
        val cutList = list.substring(1, list.length - 1)
        var depth = 0
        var tmp: String = ""
        var result: Seq[String] = Seq()
        for(i <- 0 to cutList.length - 1) {
            if(cutList(i) == '[') {
                depth += 1
            } else if(cutList(i) == ']') {
                depth -= 1
            } else if(cutList(i) == ',' && depth == 0) {
                result = result :+ tmp
                tmp = ""
            } else {
                tmp += cutList(i)
            }
        }
        result = result :+ tmp
        result
    }

    def compare(left: Seq[String], right: Seq[String]): Boolean = {
        true
    }

    def printReadFile(filename: String): Unit = {
        val file = readInput(filename)
        file.foreach(pair => {
            val pair1Split = splitList(pair(0))
            val pair2Split = splitList(pair(1))
            pair1Split.foreach(ch => println(" " + ch))
            pair2Split.foreach(ch => println(" " + ch))
            compare(pair1Split, pair2Split)
        })
    }

    def main(args: Array[String]): Unit = {
        if(args.length == 0) {
            throw new Exception("You should provide a testing file")
        }
        printReadFile(args(0))
    }

}