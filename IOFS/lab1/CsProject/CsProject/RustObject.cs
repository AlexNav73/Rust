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
        private IntPtr _ptr;

        public delegate void EnumerateCallBack(uint x, double y);

        public delegate void CountConstCallBack(double c);

        public delegate void GraphCallBack(int x, int y);


        [DllImport("External\\lab1.dll")]
        private static extern IntPtr CreateWordCounter(string s);

        [DllImport("External\\lab1.dll")]
        private static extern void DeleteWordCounter(IntPtr ptr);

        [DllImport("External\\lab1.dll")]
        private static extern void Enumerate(IntPtr ptr, MulticastDelegate predicate);
        [DllImport("External\\lab1.dll")]
        private static extern void CountConst(IntPtr ptr, MulticastDelegate predicate);
        [DllImport("External\\lab1.dll")]
        private static extern void Graph(IntPtr ptr, MulticastDelegate predicate);

        public RustObject(string path)
        {
            _ptr = CreateWordCounter(path);
        }

        public void Enumerate(EnumerateCallBack predicate) { Enumerate(_ptr, predicate);}
        public void CountConst(CountConstCallBack predicate) { CountConst(_ptr, predicate);}
        public void Graph(GraphCallBack predicate) { Graph(_ptr, predicate); }

        ~RustObject()
        {
            DeleteWordCounter(_ptr);
        }
    }
}
