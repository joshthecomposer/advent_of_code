#nullable disable warnings
string[] lines = System.IO.File.ReadAllLines("input.txt");

Directory rootDir = new Directory("/", null);
Directory currentDir = rootDir;

foreach(string l in lines)
{
    string[] split = l.Split(" ");
    int yes;
    if (split[1] == "cd")
    {
        if (split[2] == "/")
        {
            currentDir = rootDir;
        }
        else if (split[2] == "..")
        {
            currentDir = currentDir.Parent;
        }
        else
        {
            string targetName = split[2];
            if (currentDir.Directories.FirstOrDefault(d=>d.Name == targetName) is not { } targetDirectory)
            {
                Directory newDir = new Directory(targetName, currentDir);
                currentDir.Directories.Add(newDir);
                targetDirectory = newDir;
            }
            currentDir = targetDirectory;
        }
    }
    else
    {
        if (split[0] == "dir")
        {
            if (!currentDir.ContainsDirectory(split[1]))
            {
                Directory newDir = new Directory(split[1], currentDir);
                currentDir.Directories.Add(newDir);
            }
        }
        else if (Int32.TryParse(split[0], out yes))
        {
            if (!currentDir.ContainsFile(split[1]))
            {
                File newFile = new File(split[1], yes);
                currentDir.Files.Add(newFile);
            }
        }
    }
}

int max = 100000;
currentDir = rootDir;
List<Directory> smallDirs = new List<Directory>();

TraverseDirs(rootDir);

Console.WriteLine(smallDirs.Sum(d => d.TotalSize));

void TraverseDirs(Directory source)
{
    if (source.TotalSize <= max)
    {
        smallDirs.Add(source);
    }
    foreach (Directory d in source.Directories)
    {
        TraverseDirs(d);
    }
}

class File
{
    public string? Name { get; set; }
    public long Size { get; set; }

    public File(string name, long size)
    {
        Name = name;
        Size = size;
    }
}

class Directory
{
    public string? Name { get; set; }
    public Directory? Parent { get; set; }
    public List<Directory> Directories { get; set; } = new List<Directory>();
    public List<File> Files = new List<File>();

    private long? _totalSize = null;

    public long TotalSize
    {
        get
        {
            if (_totalSize != null)
            {
                return _totalSize.Value;
            }
            long size = 0;
            foreach (File file in Files)
            {
                size += file.Size;
            }
            foreach (Directory dir in Directories)
            {
                size += dir.TotalSize;
            }
            _totalSize = size;
            return _totalSize.Value;
        }
    }

    public Directory(string name, Directory parent)
    {
        Name = name;
        Parent = parent;
    }

    public bool ContainsDirectory(string dName) => Directories.Any(d => d.Name == dName);

    public bool ContainsFile(string fName) => Files.Any(f => f.Name == fName);
}
