using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace CsProject
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }

        private void button1_Click(object sender, EventArgs e)
        {
            foreach (string file in Directory.EnumerateFiles(Environment.CurrentDirectory, "*.txt"))
            {
                RustObject rs = new RustObject(file);
                rs.Enumerate((x, y) => chart1.Series[0].Points.AddXY(x, y));
//                rs.CountConst(c => Console.WriteLine("C: {0}", c));
            }
        }
    }
}
