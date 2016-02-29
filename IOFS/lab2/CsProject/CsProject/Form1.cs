using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace CsProject
{
    public partial class Form1 : Form
    {
        private const string Directory = @"D:\Programms\Rust\IOFS\lab2\texts";

        private RustObject _obj = null;

        public Form1()
        {
            InitializeComponent();

            if (System.IO.Directory.Exists(Directory))
            {
                _obj = new RustObject(Directory);
                listBox1.Items.AddRange(System.IO.Directory.EnumerateFiles(Directory).ToArray());
            }
        }

        private void button1_Click(object sender, EventArgs e)
        {
            if (!string.IsNullOrEmpty(textBox1.Text))
            {
                MessageBox.Show(_obj.Search(textBox1.Text), "Search results", MessageBoxButtons.OK);
            }
        }
    }
}
