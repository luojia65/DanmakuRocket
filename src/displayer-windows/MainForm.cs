using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using DanmuDisplayerWin;
using WebSocketSharp;

namespace DanmuDisplayer
{
    public partial class MainForm : Form
    {
        public MainForm()
        {
            InitializeComponent();
        }

        private WebSocket ws;

        private void ShowMessageForm(string msg) {
            if (string.IsNullOrWhiteSpace(msg))
            {
                return;
            }

            var form = new DanmakuForm(msg);
            form.Show();
        }

        private void MainForm_Load(object sender, EventArgs e)
        {

            ws = new WebSocket("ws://live.luojia.cc:11030");
            ws.OnMessage += (ss, ee) =>
            {
                var str = ee.Data;
                Action<string> action = (s) =>
                {
                    LabelState.Text = "Message received! " + s;
                    ShowMessageForm(s);
                };
                Invoke(action, str);
            };
            ws.Connect();
            LabelState.Text = "Connected to server!";
        }

        private void MainForm_Closing(object sender, EventArgs e)
        {
            LabelState.Text = "Disconnected from server!";
            ws.Close();
        }
        
    }
}
