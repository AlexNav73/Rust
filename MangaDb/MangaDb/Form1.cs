using MangaDb.Modules;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace MangaDb
{
    public partial class Form1 : Form
    {
        private Conveyor _conveyer = null;

        public Form1()
        {
            InitializeComponent();

            _conveyer = new Conveyor()
                .RegisterModule(new Downloader())
                .RegisterModule(new Parser());
        }

        private async void button1_Click(object sender, EventArgs e)
        {
            await _conveyer.Process();
        }
    }
}
