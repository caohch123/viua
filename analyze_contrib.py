import csv

METRICS = ['commits', 'files', 'added', 'deleted', 'churn', 'net',
           'prs_opened', 'prs_merged', 'pr_reviews', 'issues',
           'key_touches', 'fix_commits', 'docs_changes']

WEIGHTS = {
    'added': 0.25,
    'commits': 0.10,
    'files': 0.10,
    'churn': 0.05,
    'prs_opened': 0.15,
    'prs_merged': 0.10,
    'pr_reviews': 0.10,
    'issues': 0.05,
    'key_touches': 0.10,
}


def label(score):
    if score >= 0.75:
        return '核心贡献者'
    if score >= 0.50:
        return '主要贡献者'
    if score >= 0.25:
        return '有意义贡献者'
    return '较少/边缘贡献者'


def main():
    with open('metrics.csv', newline='', encoding='utf-8') as f:
        rows = list(csv.DictReader(f))

    for m in METRICS:
        vals = [float(r[m]) for r in rows]
        mmin, mmax = min(vals), max(vals)
        for r in rows:
            v = float(r[m])
            r['norm_' + m] = 1.0 if mmax == mmin else (v - mmin) / (mmax - mmin)

    active = {k: w for k, w in WEIGHTS.items()
              if any(float(r[k]) != 0 for r in rows)}
    total_w = sum(active.values())
    active = {k: w / total_w for k, w in active.items()}

    dropped = sorted(set(WEIGHTS) - set(active))
    if dropped:
        print(f"[info] 全零指标已剔除并重归一化权重: {', '.join(dropped)}")
    print(f"[info] 实际权重: " +
          ", ".join(f"{k}={w:.3f}" for k, w in active.items()))
    print()

    for r in rows:
        r['score'] = sum(r['norm_' + k] * w for k, w in active.items())

    rows.sort(key=lambda x: x['score'], reverse=True)

    with open('normalized_metrics.csv', 'w', newline='', encoding='utf-8') as f:
        fields = ['author'] + ['norm_' + m for m in METRICS]
        w = csv.DictWriter(f, fieldnames=fields, extrasaction='ignore')
        w.writeheader()
        for r in rows:
            w.writerow({k: (f"{r[k]:.4f}" if k != 'author' else r[k])
                        for k in fields})

    with open('final_scores.csv', 'w', newline='', encoding='utf-8') as f:
        w = csv.writer(f)
        w.writerow(['rank', 'author', 'score', 'label'])
        for i, r in enumerate(rows, 1):
            w.writerow([i, r['author'], f"{r['score']:.3f}", label(r['score'])])

    width = 40
    print(f"{'Rank':<5}{'Author':<15}{'Score':<8}{'Label':<14}构成条")
    for i, r in enumerate(rows, 1):
        bar = '#' * round(r['score'] * width)
        print(f"{i:<5}{r['author']:<15}{r['score']:<8.3f}"
              f"{label(r['score']):<14}{bar}")

    print()
    print("得分构成 (指标 x 权重):")
    for r in rows:
        parts = [f"{k}:{r['norm_' + k] * w:.3f}" for k, w in active.items()]
        print(f"  {r['author']}: " + " + ".join(parts))


if __name__ == '__main__':
    main()
