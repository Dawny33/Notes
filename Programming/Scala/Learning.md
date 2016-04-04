- Scala compiles to Java bytecode
- ##### “var" vs “val":

    - val is immutable, while var is mutable.
    - Example: Look at this example for better understanding:
    - http://java.sanaulla.info/2009/07/02/val-versus-var-in-scala/
- ##### Normal functions

    - Normal functions can be built by using def.
    - For example, a simple adder function can be: def AddTwo(x:Int, y:Int): Int = x + y
- ##### Anonymous functions

    - (x:Int) => x + 1 (If this is executed as res1, then it can be evoked with the name “res1")
    - Example: res1(3) would give out the value 4.
    - They can also be passed around as values also.
    - Example: val AddOne = (x: Int) => x + 1
    - Now, the value “addOne" can be evoked a function. Example: addOne(3) would return the value 4
- ##### Partial Applications

    - Useful post: http://stackoverflow.com/q/8000903/4993513 and this: http://www.slideshare.net/normation/scala-dreaded
    - Discouraged in idiomatic Scala.
    - Not related:  Prefer “val" against “var"
    - These are the uses of the “_” according to the slideshare link above:
        - Importing all: For importing everything from a package: import scala.util.control.Breaks._
        - For defining values. Int takes 0, Float takes null, and float takes 0.0. Example: “ var i:Int = _ “
        - Unused variables: When the variable doesn’t need to be initialized. Ex: (1 to 5) foreach {_ => println(“Hello")}
        - Anonymous Parameters: For anonymous parameters like: (1 to 10)  map { x => x+1 } is same as (1 to 10) map { _+1 }
- ##### Curried functions

    - A curried function is one where multiple arguments are described by a series of one-argument functions.  (https://hughfdjackson.com/javascript/why-curry-helps/)
    - It allows the programmer to apply some arguments now, and some others later.
    - For example, this function: def multiply(x: Int)(y: Int): Int = x * y is a curried function which takes in two arguments x and y
    - Now, if we run the function with a partial application, it would create a new function: multiply(2)_, day res1. Now, by executing res1(5) would give the result 10.
    - Even normal functions can also be curried, and this is how to do it.
        - Consider a function called adder which adds two numbers: def adder(x:Int, y:Int): Int = x+y.
        - Now, the function can be curried as follows: val CurriedFunc = (adder _).curried
        - Now, the curried function can be used as explained in the above bullet, where the curried functions are explained. Example: executing CurriedFunc(4) creates a new function, say res8. Now, use res8 to add any number with 4.
- ##### Variable Length Arguments

    - The map function can also be learnt while understanding multiple argument functions
    - For defining a function with multiple parameters of the same type, a * can be used
    - Example: def capitalize(args: String*) = { args.map{ arg => arg.capitalize } }
    - The above can also be written as: def capitalize(args: String*) = { args.map{ _.capitalize } } making use of partial applications
    - An other application for integer values is as follows: def sum(args: Int*) = { args.map{ _*3 } }
- ##### Classes
    - Here is an example of a class:
class Calculator{
     val brand = “HP”
     def add(x:Int, y:Int): Int = x + y
}
    - The above class’s instance can be created as follows: val calc = new Calculator
    - So, all the operations and functions defined in the class can be used as follows:
        - calc.brand would return “HP"
        - calc.add(3, 4) would return 7
        - Methods are functions that access the state of the class
- ##### Constructors
    - Constructors are the code outside method definitions in a class
    - An example constructor can be:
    - class Student(name: String)
{
  val sex:String = if (name == “Raj") { “male" }
      else if (name == “Time"){ “female" }
      else { “Not known" }

                          def add(x:Int, y:Int): Int = x + y
                        }

    - The class instance can be created in the old fashioned way: val student = new Student(“Raj”). The part other than the “def" is the constructor
    - Now, the student’s sex can be seen by: student.sex
- ##### Method vs Function
    - A method is a part of a class, whereas the function is a complete object. (Helpful syntax: http://jim-mcbeath.blogspot.in/2009/05/scala-functions-vs-methods.html)
    - A method can’t be the final value, however a function can.
    - Example: def add(x:Int, y:Int): Int = x + y is a method.  And val Add = (x:Int, y:Int) => x + y is a function. (https://dzone.com/articles/revealing-scala-magician%E2%80%99s)
    - Parameter list is optional for methods, but mandatory for functions. However, a function’s parameter list can be empty.
    - Method name means invocation, but a function name means the function itself.
    - ##### Inheritance

        - The inheritance property can be used to inherit and extend classes and create new and better classes. This is demonstrated in the following example below:
        - class ScientificCalc(brand: String) extends Calculator(brand) {
        def log(m: Double, base: Double) = math.log(m) / math.log(base)
    }
        - This is also called subclassing (Helpful article: http://www.scala-lang.org/old/node/125)
    - ##### Method Overloading

        - It is a feature that allows a class to have two or more methods having same name, if their argument lists are different.
        - Example:
    class MoreScientificCalculator(brand:String) extends ScientificCalculator(brand) {
        def log(m: Int): Double = log(m, math.exp(1))
    }
    - ##### Abstract Class

        - Abstract classes are like normal classes as they take in arguments and define methods. But, they do not implement those methods.
        - They let subclasses extend them and use and define those methods. So, instances of an abstract class cannot be created.
        - So, a subclass which extends an abstract class should define the methods. An example is in the next point:
        - abstract class Shape {
        def getArea(): Int
    }
    class Circle(r: Int) extends Shape {
        def getArea():Int = { r*r*3 }
    }

    - ##### Traits

        - Traits are a collection of fields and behaviours which can be inherited into/extended to classes. Below is an example:
        - trait Regularity {
        val attendance: Int
    }
    trait Intelligence {
        val physics_score: Int
    }

        - These traits can be extended the following way:
    class Student extends Regularity {
         val attendance = 26
        - Multiple traits can be extended into, by using with. Here is an example of the same:
        - class Student extends Regularity with Intelligence {
        val attendance = 26
        val physics_score = 89
    }
    - ##### Choosing between Abstract Classes and Traits

        - Choose traits whenever possible, as a class can extend/inherit multiple traits, however can only extend one abstract class.
        - If you want constructor parameters, choose abstract classes over traits, as the latter can only have values and cannot have methods or constructors.
    - ##### Types
        - Functions can be generic and can work on any type. They need not be defined by specific types like Int, String, etc.
        - So, such parameters can be included in square brackets. The following example explains how:
    trait Cache[K, V] {
       def get(key: K): V
       def put(key: K, value: V)
       def delete(key: K)
    }

        - Methods can also have type parameters introduced. Here is an example below:
    def remove[k](key: K)
