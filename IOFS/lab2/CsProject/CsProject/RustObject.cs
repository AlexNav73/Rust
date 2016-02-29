using System;
using System.Collections.Generic;
using System.Linq;
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

        public RustObject(string dir)
        {
            _ptr = CreateWordSearcher(dir);
        }

        public string Search(string text) { return Search(_ptr, text); }

        ~RustObject()
        {
            DeleteWordSearcher(_ptr);
        }
    }
}
