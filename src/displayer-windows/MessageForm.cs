using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace DanmuDisplayer
{
    public partial class MessageForm : Form
    {
        private String msg;
        public MessageForm(String msg)
        {
            this.msg = msg;
            InitializeComponent();
        }
        
        private void MainForm_Load(object sender, EventArgs e)
        {
            LabelMessage.Text = msg;
            Left = SystemInformation.PrimaryMonitorSize.Width;
            ShowInTaskbar = false;//窗体不出现在Windows任务栏中
            TransparencyKey = Color.FromArgb(233, 233, 233);
            TopMost = true;//使窗体始终在其它窗体之上
        }

        private void Ticker_Tick(object sender, EventArgs e)
        {
            if (Left > -LabelMessage.Width)
            {
                Left = Location.X - 3;
            } else
            {
                Close();
            }
        }
    }
}
