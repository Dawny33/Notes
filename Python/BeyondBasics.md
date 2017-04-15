

**Packages:**
1. All packages are of the type “module”
2. When done <>.__path__, it returns all the paths where the package searches for, for it’s nested modules

**Imports from sys.path:**
1. sys.path is the list of directories Python searches for modules
2. If the directory is not in sys.path, it will not be considered by Python, which mean we can’t import any modules in that dir. 
3. For adding it, do “export PYTHONPATH=<path/to/folder>”


**Implementing Packages**
1. For creating a package, create a folder inside any sys.path folder
2. Then, create a __init__.py inside that folder.
3. ^^ This makes the package/folder a module.  It is called a “package init” file
4. Example, create a folder “reader” and put in an empty __init__.py file.
5. Now, start a Py prompt and do import reader. We can now import reader as a module!  Yay!!
6. “reader.__file__” gives the path of the reader’s __init__.py
7. When we import reader (i.e. when we d import reader), the __init__.py is being executed


**Relative Imports**
1. Imports can also use relative path to modules in the same package.
2. `from .b import B`  <— For the package containing current module
3. `from ..a import A` <— For package containing the package containing the current module.
4. Can reduce typing in deeply nested package structure
5. Promote certain forms of modifiability
6. Can aid package renaming and refactoring.
7. However, the general advice is to avoid using them whenever possible

**Functions**
1. Functional arguments are of 2 types:
    1. Positional Argument - placed in a particular order
    2. Keyword argument - takes some value
    3. Keyword arguments are placed after positional arguments
    4. `< func_name >(arg1, arg2 = 1.6)` <— `arg1` is a positional argument, `arg2` is a keyword argument

**Callable Instances**
1. __call___ <— can be used to define classes which when instantiated, can be called using regular function calling syntax
2. Useful when we want functions to maintain state between calls and needs some methods/actions and queries to modify that state.
3. Example: Check the code in the `Resolver` folder
4. `from resolver import Resolver`
    1. `resolve = Resolver()`
    2. `resolve.has_host('google.com')`
    3. `resolve.clear()`
4. In this example, we can maintain the state of the cache data-structure(list, in this case), also along with modifying it whenever we want.