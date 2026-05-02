import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

/// A collapsible banner displayed at the top of the app below the app bar.
class TopBanner extends StatefulWidget {
  final double expandedHeight;

  const TopBanner({Key? key, this.expandedHeight = 120}) : super(key: key);

  @override
  _TopBannerState createState() => _TopBannerState();
}

class _TopBannerState extends State<TopBanner> with SingleTickerProviderStateMixin {
  bool _expanded = true;
  String _missionText = '';

  void _toggle() {
    setState(() {
      _expanded = !_expanded;
    });
  }

  @override
  void initState() {
    super.initState();
    _loadMissionText();
  }

  Future<void> _loadMissionText() async {
    try {
      final txt = await rootBundle.loadString('assets/top_banner_mission_statement.txt');
      setState(() {
        _missionText = txt;
      });
    } catch (e) {
      setState(() {
        _missionText = 'Unable to load mission statement.';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final collapsedHeight = 40.0;
    final duration = const Duration(milliseconds: 300);

    return AnimatedContainer(
      duration: duration,
      height: _expanded ? widget.expandedHeight : collapsedHeight,
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.centerLeft,
          end: Alignment.centerRight,
          colors: [Colors.indigo.shade900, Colors.indigo.shade600],
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Text(
                  'Peace On Earth 🕊️  P.O.E. — a nod to Edgar Allen as I\'m creating it through solutions (not Utopian) in Baltimore. =^_~=',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w700,
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              IconButton(
                icon: Icon(_expanded ? Icons.expand_less : Icons.expand_more, color: Colors.white),
                onPressed: _toggle,
                tooltip: _expanded ? 'Collapse' : 'Expand',
              ),
            ],
          ),
          const SizedBox(height: 8),
          Expanded(
            child: AnimatedCrossFade(
              firstChild: const SizedBox.shrink(),
              secondChild: SingleChildScrollView(
                child: _missionText.isEmpty
                    ? const Padding(
                        padding: EdgeInsets.all(8.0),
                        child: SizedBox(height: 24, child: Center(child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white70))),
                      )
                    : Padding(
                        padding: const EdgeInsets.only(right: 8.0),
                        child: Text(
                          _missionText,
                          style: const TextStyle(
                            color: Colors.white70,
                            fontSize: 12,
                          ),
                        ),
                      ),
              ),
              crossFadeState: _expanded ? CrossFadeState.showSecond : CrossFadeState.showFirst,
              duration: duration,
            ),
          ),
        ],
      ),
    );
  }
}
