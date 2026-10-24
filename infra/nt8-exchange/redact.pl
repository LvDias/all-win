#!/usr/bin/perl
# Redacts secrets and personal data from NinjaTrader/All Win text logs. Used on the Mac before sending and on the
# server before reading (check mode). Standard perl only (macOS and Ubuntu ship it).
#   redact.pl FILE...            write the redacted text to stdout
#   redact.pl --check FILE...    print nothing of the content; exit 1 if any rule would change a file
# Counts per rule go to stderr (never the matched text).
use strict;
use warnings;

my $check = 0;
if (@ARGV && $ARGV[0] eq '--check') { $check = 1; shift @ARGV; }

# [name, regex, replacement]. Order matters: specific shapes first.
my @rules = (
  ['private-key-line', qr/-----BEGIN [A-Z ]*PRIVATE KEY-----.*/,              '<redacted:private-key>'],
  ['all-win-key',      qr/\baw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{20,}/,       '<redacted:key>'],
  ['tailscale-key',    qr/\btskey-[A-Za-z0-9-]{10,}/,                          '<redacted:key>'],
  ['bearer-basic',     qr/\b(Bearer|Basic)\s+[A-Za-z0-9._~+\/=-]{8,}/i,        '$1 <redacted>'],
  ['secret-assignment',qr/\b((?:authorization|password|passwd|pwd|secret|token|api[_-]?key|apikey|license[_-]?key|serial(?:[_ ]?number)?|private[_-]?key|credential)s?\s*[:=]\s*)(?!<redacted)["']?[^\s,;"'&]+/i, '$1<redacted>'],
  ['url-credential',   qr/(\b[a-z][a-z0-9+.-]*:\/\/)[^\s\/@:]+:[^\s\/@]+@/i,   '$1<redacted>@'],
  ['email',            qr/\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b/,  '<redacted:email>'],
  ['tailnet-host',     qr/\b[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)*\.ts\.net\b/i,    '<redacted:host>'],
  ['ipv4',             qr/\b(?!127\.0\.0\.1\b|0\.0\.0\.0\b)(?:\d{1,3}\.){3}\d{1,3}\b/, '<redacted:ip>'],
  ['ipv6',             qr/\b(?:[0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}\b/,       '<redacted:ip>'],
  ['hex-blob',         qr/\b[A-Fa-f0-9]{32,}\b/,                               '<redacted:hex>'],
  ['long-token',       qr/\b(?=[A-Za-z0-9+_-]*\d)(?=[A-Za-z0-9+_-]*[A-Za-z])[A-Za-z0-9+_-]{40,}={0,2}/, '<redacted:token>'],
  ['windows-user',     qr/\b([A-Za-z]:\\Users\\)[^\\\s"'<>|]+/i,               '$1<user>'],
  ['account-name',     qr/(\baccount(?:[ _]?(?:name|id|number))?\s*[:=]?\s*["']?)(?!Sim\d+\b|Playback\d+\b|<redacted)([A-Za-z0-9][A-Za-z0-9_-]{3,})/i, '$1<redacted:account>'],
);

my %count;
my $changed_any = 0;
for my $file (@ARGV) {
  open(my $fh, '<:raw', $file) or do { warn "redact: cannot read $file\n"; $changed_any = 1; next };
  my $file_changed = 0;
  while (my $line = <$fh>) {
    for my $r (@rules) {
      my ($name, $re, $rep) = @$r;
      my $n = 0;
      if ($rep =~ /\$1/) { $n = ($line =~ s/$re/my $g=$1; ($rep =~ s{\$1}{$g}r)/ge) }
      else               { $n = ($line =~ s/$re/$rep/g) }
      if ($n) { $count{$name} += $n; $file_changed = 1 }
    }
    print $line unless $check;
  }
  close $fh;
  $changed_any ||= $file_changed;
}
print STDERR "redact: $_ x$count{$_}\n" for sort keys %count;
exit($check && $changed_any ? 1 : 0);
