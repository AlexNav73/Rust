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
using System.Windows.Forms.DataVisualization.Charting;

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

            int fileNumber = 0;
            foreach (string file in Directory.EnumerateFiles(Environment.CurrentDirectory, "*.txt"))
            {
                RustObject rs = new RustObject(file);
                string fileName = Path.GetFileNameWithoutExtension(file);

                chart1.Series.Add(new Series(fileName) { ChartType = SeriesChartType.Spline });
                chart2.Series.Add(new Series(fileName) { ChartType = SeriesChartType.Spline });

                rs.Graph((x, y) => chart1.Series[fileNumber].Points.AddXY(x, y));

                rs.Enumerate((x, y) => chart2.Series[fileName].Points.AddXY(x, y));

                double totalC = 0;
                int count = 0;

                rs.CountConst(c =>
                {
                    totalC += c;
                    count++;
                });

                label1.Text += string.Format("{0}: {1}\n", fileName, totalC / count);
                fileNumber += 1;
            }
        }
    }
}
