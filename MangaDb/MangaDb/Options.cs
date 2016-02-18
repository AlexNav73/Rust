using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using MangaDb.Configurations;

namespace MangaDb
{
    public partial class Options : Form
    {
        public Options()
        {
            InitializeComponent();
        }

        private void button1_Click(object sender, EventArgs e)
        {
            var conf = Helpers.ConfigurationHelper.Deserialize<MainConfig>();

            var form = (Control) this;
            foreach (Control control in form.Controls)
            {
                var prop = typeof(MainConfig).GetProperty(control.Name);

                if (prop != null) prop.SetValue(conf, control.Text);
            }

            Helpers.ConfigurationHelper.Serialize(conf);
        }
    }
}
