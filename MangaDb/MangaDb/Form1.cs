using MangaDb.Configurations;
using MangaDb.Entities;
using MangaDb.Helpers;
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
        public Form1()
        {
            InitializeComponent();
        }

        private async void button1_Click(object sender, EventArgs e)
        {
            var conveyer = new Conveyor()
                .RegisterModule(new Downloader())
                .RegisterModule(new Parser())
                .RegisterModule(new NewRecordsFilter());

            List<ListEntry> list = (List<ListEntry>)await conveyer.Process();
            foreach (ListEntry item in list)
            {
                listBox1.Items.Add(item.Name);
            }
        }
    }
}
