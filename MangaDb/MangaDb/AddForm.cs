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
using MangaDb.Configurations;
using MangaDb.Entities;
using MangaDb.Extensions;
using MangaDb.Helpers;
using MangaDb.Repositories.Implementations;
using CsvHelper = MangaDb.Helpers.CsvHelper;

namespace MangaDb
{
    public partial class AddForm : Form
    {
        private readonly Type _entryType = null;

        public AddForm()
        {
            InitializeComponent();

            _entryType = AppSettings.EntryType;
        }

        public void OnSave(object sender, EventArgs e)
        {
            var form = (Control) this;
            var entry = Activator.CreateInstance(_entryType);

            foreach (Control control in form.Controls)
            {
                var prop = _entryType.GetProperty(control.Name);

                if (prop != null) prop.SetValue(entry, control.Text);

                control.Text = "";
            }

            var conf = ConfigurationHelper.Deserialize<MainConfig>(AppSettings.MainConfigPath);
            Helpers.CsvHelper helper = new Helpers.CsvHelper();
            helper.AppendRecords(conf.FilePath, _entryType, new[] { entry });
        }

        private void AddForm_Load(object sender, EventArgs e)
        {
            var form = (Control) this;

            int width = 200;
            int left = 50;
            int top = 10;

            _entryType.GetProperties()
                .ForEach(p =>
                {
                    var label = new Label() {Name = p.Name + "_label", Text = p.Name, Left = left, Top = top};
                    form.Controls.Add(label);
                    form.Controls.Add(new TextBox()
                    {
                        Name = p.Name, 
                        Width = width, 
                        Left = label.Left + label.Width, 
                        Top = top
                    });

                    top += label.Height + 10;
                });

            var button = new Button() { Name = "save", Left = left, Top = top + 30, Text = "Save" };
            button.Click += OnSave;
            form.Controls.Add(button);
        }
    }
}
