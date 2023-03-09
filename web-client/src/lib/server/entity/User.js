export default function (sequelize, DataTypes) {
	return sequelize.define(
		'User',
		{
			id: { type: DataTypes.BIGINT, primaryKey: true }
		},
		{
			sequelize,
			tableName: 'user',
			timestamps: false
		}
	);
}
