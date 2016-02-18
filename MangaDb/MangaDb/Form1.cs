﻿using MangaDb.Configurations;
using MangaDb.Entities;
using MangaDb.Formatters.Implementations;
using MangaDb.Helpers;
using MangaDb.Modules;
using MangaDb.Modules.Implementations;
using MangaDb.Repositories.Implementations;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
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

        private async void viewToolStripMenuItem_Click(object sender, EventArgs e)
        {
            var conveyer = new Conveyor()
                .RegisterModule(new Downloader())
                .RegisterModule(new Parser())
                .RegisterModule(new UpdateDb(new RecordRepository()))
                .RegisterModule(new Format(new HtmlFormatter()));

            webBrowser1.DocumentText = (string)await conveyer.Process();
        }

        private static bool _willNavigate;

        private void webBrowser1_Navigating(object sender, WebBrowserNavigatingEventArgs e)
        {
            if (!_willNavigate)
            {
                _willNavigate = true;
                return;
            }

            e.Cancel = true;
            Process.Start(new ProcessStartInfo() { FileName = e.Url.ToString() });
        }
    }
}
