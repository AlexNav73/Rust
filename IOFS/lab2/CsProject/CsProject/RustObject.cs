using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Resources;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace CsProject
{
    public class RustObject
    {
        private readonly IntPtr _ptr = IntPtr.Zero;

        [DllImport("Extern\\lab2.dll")]
        private static extern IntPtr CreateWordSearcher(string dir);
        [DllImport("Extern\\lab2.dll")]
        private static extern void DeleteWordSearcher(IntPtr obj);
        [DllImport("Extern\\lab2.dll")]
        private static extern string Search(IntPtr obj, string searchString);

        private string _path = null;

        public RustObject(string dir)
        {
            _ptr = CreateWordSearcher(dir);
            _path = dir;
        }

        public string Search(string text)
        {
            string res = null;

            try
            {
                return Search(_ptr, text);
            }
            catch (Exception e)
            {
                foreach (string file in Directory.EnumerateFiles(_path, "*.txt"))
                {
                    string allText = File.ReadAllText(file);
                    string[] words = allText.Split(new[] {' '}, StringSplitOptions.RemoveEmptyEntries);
                    res = words.FirstOrDefault(w => w.Equals(text));
                    if (res != null)
                        return file;
                }
            }

            return null;
        }

        ~RustObject()
        {
            DeleteWordSearcher(_ptr);
        }
    }
}
