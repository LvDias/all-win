// All Win — NT8 AddOn entry point (minimal version): starts the host runtime and adds Control Center → New → All Win.
// Compiled by NinjaTrader on import (C# 7.3). The window shows the connection state and the diagnostics buttons;
// it never asks for or shows a token.
#region Using declarations
using System;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Threading;
using NinjaTrader.Gui;
using NinjaTrader.Gui.Tools;
#endregion

namespace NinjaTrader.NinjaScript.AddOns
{
    public class AllWinAddOn : AddOnBase
    {
        private NTMenuItem newMenu;
        private NTMenuItem menuItem;

        protected override void OnStateChange()
        {
            if (State == State.SetDefaults)
            {
                Name = "AllWin";
                Description = "All Win " + AllWinRuntime.Version;
            }
            else if (State == State.Active)
            {
                AllWinDiagnostics.Instance.Start();
                AllWinRuntime.Instance.Start();
            }
            else if (State == State.Terminated)
            {
                AllWinRuntime.Instance.Stop();
                AllWinDiagnostics.Instance.Stop();
            }
        }

        protected override void OnWindowCreated(Window window)
        {
            ControlCenter cc = window as ControlCenter;
            if (cc == null) return;
            newMenu = cc.FindFirst("ControlCenterMenuItemNew") as NTMenuItem;
            if (newMenu == null)
            {
                AllWinDiagnostics.Instance.Log("ui", "Control Center menu New not found");
                return;
            }
            menuItem = new NTMenuItem { Header = "All Win", Style = Application.Current.TryFindResource("MainMenuItem") as Style };
            newMenu.Items.Add(menuItem);
            menuItem.Click += OnMenuClick;
        }

        protected override void OnWindowDestroyed(Window window)
        {
            if (menuItem == null || !(window is ControlCenter)) return;
            if (newMenu != null && newMenu.Items.Contains(menuItem)) newMenu.Items.Remove(menuItem);
            menuItem.Click -= OnMenuClick;
            menuItem = null;
        }

        private void OnMenuClick(object sender, RoutedEventArgs e)
        {
            try { new AllWinWindow().Show(); }
            catch (Exception ex) { AllWinDiagnostics.Instance.Log("ui", "window failed: " + ex); }
        }
    }

    /// <summary>Plain WPF window: connection state (refreshed every second) and the diagnostics buttons.</summary>
    public class AllWinWindow : Window
    {
        private readonly TextBlock state;
        private readonly TextBox output;
        private readonly DispatcherTimer timer;

        public AllWinWindow()
        {
            Title = "All Win " + AllWinRuntime.Version;
            Width = 900;
            Height = 600;
            var root = new DockPanel();
            state = new TextBlock { Margin = new Thickness(8), FontWeight = FontWeights.Bold, TextWrapping = TextWrapping.Wrap };
            DockPanel.SetDock(state, Dock.Top);
            root.Children.Add(state);
            var buttons = new WrapPanel();
            DockPanel.SetDock(buttons, Dock.Top);
            root.Children.Add(buttons);
            output = new TextBox { IsReadOnly = true, AcceptsReturn = true, TextWrapping = TextWrapping.NoWrap, FontFamily = new System.Windows.Media.FontFamily("Consolas"),
                VerticalScrollBarVisibility = ScrollBarVisibility.Auto, HorizontalScrollBarVisibility = ScrollBarVisibility.Auto };
            root.Children.Add(output);
            Content = root;

            AddButton(buttons, "Listar contas / List accounts", () => AllWinDiagnostics.Instance.ListAccounts());
            AddButton(buttons, "Mostrar log / Show log", () => AllWinRuntime.Instance.Log == null ? "" : AllWinRuntime.Instance.Log.Recent(200));
            AddButton(buttons, "Mostrar eventos / Show events", () => AllWinDiagnostics.Instance.Events());
            AddButton(buttons, "Listar execuções / List executions", () => AllWinDiagnostics.Instance.ListExecutions());
            AddButton(buttons, "Exportar diagnóstico / Export diagnostics", () => AllWinDiagnostics.Instance.ExportDiagnostics());

            output.Text = "All Win " + AllWinRuntime.Version + Environment.NewLine + "Núcleo / Core: " + AllWinRuntime.Endpoint
                + Environment.NewLine + "Logs: " + AllWinRuntime.LogDir;
            timer = new DispatcherTimer { Interval = TimeSpan.FromSeconds(1) };
            timer.Tick += (s, e) => state.Text = AllWinRuntime.Instance.Status;
            timer.Start();
            state.Text = AllWinRuntime.Instance.Status;
            Closed += (s, e) => timer.Stop();
        }

        private void AddButton(Panel panel, string label, Func<string> action)
        {
            var button = new Button { Content = label, Margin = new Thickness(4), Padding = new Thickness(8, 4, 8, 4) };
            button.Click += (s, e) =>
            {
                try { output.Text = action(); }
                catch (Exception ex)
                {
                    AllWinDiagnostics.Instance.Log("ui", label + " failed: " + ex);
                    output.Text = "Erro / error: " + ex.Message;
                }
            };
            panel.Children.Add(button);
        }
    }
}
