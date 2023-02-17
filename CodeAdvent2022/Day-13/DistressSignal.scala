import scala.io.Source

object DistressSignal {

    def readInput(filename: String): Seq[Seq[String]] = {
        Source.fromFile(filename).getLines.toList.filter(_ != "").grouped(2).toList
    }

    def splitList(list: Seq[String]): Seq[String] = {
        list match {
            case Nil => Nil
            case head :: tail => head.split(",").toSeq ++ splitList(tail)
        }
    }

    def compare(left: Seq[String], right: Seq[String]): Boolean = {
        
    }

    def printReadFile(filename: String): Unit = {
        val file = readInput(filename)
        file.foreach(pair => {
            
        })
    }

    def main(args: Array[String]): Unit = {
        if(args.length == 0) {
            throw new Exception("You should provide a testing file")
        }
        printReadFile(args(0))
    }

}